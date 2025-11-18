namespace Duper;

using System.Collections;
using System.Reflection;
using Ffi;

public class DuperSerializer
{
  private static bool IsTuple(Type t)
  {
    if (t.FindInterfaces((t, _) => t.FullName == typeof(System.Runtime.CompilerServices.ITuple).FullName, null).Length > 0)
    {
      return true;
    }
    if (!t.IsGenericType)
    {
      return false;
    }
    Type t2 = t.GetGenericTypeDefinition();
    return t2 == typeof(ValueTuple<>)
      || t2 == typeof(ValueTuple<,>)
      || t2 == typeof(ValueTuple<,,>)
      || t2 == typeof(ValueTuple<,,,>)
      || t2 == typeof(ValueTuple<,,,,>)
      || t2 == typeof(ValueTuple<,,,,,>)
      || t2 == typeof(ValueTuple<,,,,,,>)
      || t2 == typeof(ValueTuple<,,,,,,,>);
  }

  public static T? Deserialize<T>(string @input)
  {
    DuperValue duperValue = Duper.Parse(input, true);
    Type t = typeof(T);
    object? value = DeserializeInner(duperValue, typeof(T));
    if (value == null)
    {
      if (!t.IsValueType || Nullable.GetUnderlyingType(t) != null)
      {
        return default;
      }
      throw new ApplicationException($"Cannot cast null to non-nullable {t}");
    }
    else
    {
      return (T)value;
    }
  }

  private static object? DeserializeInner(DuperValue duperValue, Type t)
  {
    // Null
    if (duperValue is DuperValue.Null)
    {
      return null;
    }

    // Object
    else if (duperValue is DuperValue.Object obj)
    {
      t = Nullable.GetUnderlyingType(t) ?? t;
      if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IDictionary<,>))
      {
        var generics = t.GetGenericArguments();
        Type keyType = generics[0];
        if (keyType != typeof(string))
        {
          throw new ApplicationException($"Cannot parse object to dictionary with non-string keys");
        }
        Type valueType = generics[1];
        var concreteType = typeof(Dictionary<,>).MakeGenericType(generics);
        var dict = Activator.CreateInstance(concreteType) ?? throw new ApplicationException("No constructor found for Dictionary");
        var addMethod = concreteType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for Dictionary");
        foreach (var item in obj.value)
        {
          addMethod.Invoke(dict, [item.key, DeserializeInner(item.value, valueType)]);
        }
        return dict;
      }
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IDictionary<,>))
        {
          var generics = interfaceType.GetGenericArguments();
          Type keyType = generics[0];
          if (keyType != typeof(string))
          {
            throw new ApplicationException($"Cannot parse object to dictionary with non-string keys");
          }
          Type valueType = generics[1];
          var dict = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
          var addMethod = interfaceType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for IDictionary");
          foreach (var item in obj.value)
          {
            addMethod.Invoke(dict, [item.key, DeserializeInner(item.value, valueType)]);
          }
          return dict;
        }
      }
      // Create class instance
      Dictionary<string, DuperValue> classFields = new(obj.value.Length);
      foreach (var entry in obj.value)
      {
        classFields.Add(entry.key, entry.value);
      }
      ConstructorInfo? parameterlessConstructor = null;
      foreach (var constructor in t.GetConstructors())
      {
        var parameters = constructor.GetParameters();
        if (parameters.Length == 0)
        {
          parameterlessConstructor = constructor;
        }
        else
        {
          try
          {
            List<object?> paramArray = new(parameters.Length);
            foreach (var param in parameters)
            {
              string? key = param.Name;
              Attribute[] attrs = Attribute.GetCustomAttributes(param);
              foreach (Attribute attr in attrs)
              {
                if (attr is DuperAttribute a)
                {
                  if (a.Key != null)
                  {
                    key = a.Key;
                  }
                  break;
                }
              }
              if (key == null)
              {
                continue;
              }
              paramArray.Add(DeserializeInner(classFields[key], param.ParameterType));
            }
            object instance = constructor.Invoke([.. paramArray]);
            return instance;
          }
          catch
          {
            continue;
          }
        }
      }
      if (parameterlessConstructor != null)
      {
        object instance = parameterlessConstructor.Invoke([]);
        foreach (var field in t.GetFields())
        {
          string key = field.Name;
          Attribute[] attrs = Attribute.GetCustomAttributes(field);
          foreach (Attribute attr in attrs)
          {
            if (attr is DuperAttribute a)
            {
              if (a.Key != null)
              {
                key = a.Key;
              }
              break;
            }
          }
          var item = classFields[key] ?? throw new ApplicationException($"No key {key} found in Duper object");
          field.SetValue(instance, DeserializeInner(item, field.FieldType));
        }
        foreach (var prop in t.GetProperties())
        {
          string key = prop.Name;
          Attribute[] attrs = Attribute.GetCustomAttributes(prop);
          foreach (Attribute attr in attrs)
          {
            if (attr is DuperAttribute a)
            {
              if (a.Key != null)
              {
                key = a.Key;
              }
              break;
            }
          }
          var item = classFields[key] ?? throw new ApplicationException($"No key {key} found in Duper object");
          prop.SetValue(instance, DeserializeInner(item, prop.PropertyType));
        }
        return instance;
      }
      throw new ApplicationException($"No valid constructors found for {t}");
    }

    // Array
    else if (duperValue is DuperValue.Array array)
    {
      t = Nullable.GetUnderlyingType(t) ?? t;
      if (IsTuple(t))
      {
        var tupleFields = t.GetFields();
        if (tupleFields.Length != array.value.Length)
        {
          throw new ApplicationException($"Mismatched tuple sizes: Duper has length {array.value.Length}, target has length {tupleFields.Length}");
        }
        object?[] tupleObjects = new object[tupleFields.Length];
        for (int i = 0; i < tupleFields.Length; i++)
        {
          tupleObjects[i] = DeserializeInner(array.value[i], tupleFields[i].FieldType);
        }
        var constructor = t.GetConstructor(t.GetGenericArguments());
        if (constructor == null)
        {
          throw new ApplicationException($"No constructor found for tuple {t}");
        }
        else
        {
          return constructor.Invoke(tupleObjects);
        }
      }
      else if (t.IsGenericType && (t.GetGenericTypeDefinition() == typeof(IEnumerable<>) || t.GetGenericTypeDefinition() == typeof(IList<>)))
      {
        Type itemType = t.GetGenericArguments().Single();
        var concreteType = typeof(List<>).MakeGenericType(t.GetGenericArguments());
        var list = Activator.CreateInstance(concreteType) ?? throw new ApplicationException("No constructor found for List");
        var addMethod = concreteType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for List");
        foreach (var item in array.value)
        {
          addMethod.Invoke(list, [DeserializeInner(item, itemType)]);
        }
        return list;
      }
      else if (t.IsArray)
      {
        Type itemType = t.GetElementType() ?? throw new ApplicationException($"No element type found for {t}"); ;
        var arrayListType = typeof(List<>).MakeGenericType([itemType]);
        var arrayList = Activator.CreateInstance(arrayListType) ?? throw new ApplicationException("No constructor found for List");
        var addMethod = arrayListType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for List");
        foreach (var item in array.value)
        {
          addMethod.Invoke(arrayList, [DeserializeInner(item, itemType)]);
        }
        var toArrayMethod = arrayListType.GetMethod("ToArray") ?? throw new ApplicationException("No ToArray method found for List");
        return toArrayMethod.Invoke(arrayList, null);
      }
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IList<>))
        {
          Type itemType = interfaceType.GetGenericArguments().Single();
          var list = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
          var ilist = (list as IList) ?? throw new ApplicationException("IList cast shouldn't fail");
          foreach (var item in array.value)
          {
            ilist.Add(DeserializeInner(item, itemType));
          }
          return list;
        }
      }
      throw new ApplicationException($"Cannot cast array to {t}");
    }

    // Tuple
    else if (duperValue is DuperValue.Tuple tuple)
    {
      t = Nullable.GetUnderlyingType(t) ?? t;
      if (IsTuple(t))
      {
        var tupleFields = t.GetFields();
        if (tupleFields.Length != tuple.value.Length)
        {
          throw new ApplicationException($"Mismatched tuple sizes: Duper has length {tuple.value.Length}, target has length {tupleFields.Length}");
        }
        object?[] tupleObjects = new object[tupleFields.Length];
        for (int i = 0; i < tupleFields.Length; i++)
        {
          tupleObjects[i] = DeserializeInner(tuple.value[i], tupleFields[i].FieldType);
        }
        var constructor = t.GetConstructor(t.GetGenericArguments());
        if (constructor == null)
        {
          throw new ApplicationException($"No constructor found for tuple {t}");
        }
        else
        {
          return constructor.Invoke(tupleObjects);
        }
      }
      else if (t.IsGenericType && (t.GetGenericTypeDefinition() == typeof(IEnumerable<>) || t.GetGenericTypeDefinition() == typeof(IList<>)))
      {
        Type itemType = t.GetGenericArguments().Single();
        var concreteType = typeof(List<>).MakeGenericType(t.GetGenericArguments());
        var list = Activator.CreateInstance(concreteType) ?? throw new ApplicationException("No constructor found for List");
        var addMethod = concreteType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for List");
        foreach (var item in tuple.value)
        {
          addMethod.Invoke(list, [DeserializeInner(item, itemType)]);
        }
        return list;
      }
      else if (t.IsArray)
      {
        Type itemType = t.GetElementType() ?? throw new ApplicationException($"No element type found for {t}"); ;
        var arrayListType = typeof(List<>).MakeGenericType([itemType]);
        var arrayList = Activator.CreateInstance(arrayListType) ?? throw new ApplicationException("No constructor found for List");
        var addMethod = arrayListType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for List");
        foreach (var item in tuple.value)
        {
          addMethod.Invoke(arrayList, [DeserializeInner(item, itemType)]);
        }
        var toArrayMethod = arrayListType.GetMethod("ToArray") ?? throw new ApplicationException("No ToArray method found for List");
        return toArrayMethod.Invoke(arrayList, null);
      }
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IList<>))
        {
          Type itemType = interfaceType.GetGenericArguments().Single();
          var list = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
          var ilist = (list as IList) ?? throw new ApplicationException("IList cast shouldn't fail");
          foreach (var item in tuple.value)
          {
            ilist.Add(DeserializeInner(item, itemType));
          }
          return list;
        }
      }
      throw new ApplicationException($"Cannot cast tuple to {t}");
    }

    // String
    else if (duperValue is DuperValue.String str)
    {
      if (typeof(string).IsAssignableTo(t))
      {
        return str.value;
      }
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IParsable<>))
        {
          var parseMethod = typeof(DuperSerializer).GetMethod("ParseViaGeneric", BindingFlags.NonPublic | BindingFlags.Static) ?? throw new ApplicationException("No ParseViaGeneric method found for DuperSerializer");
          parseMethod = parseMethod.MakeGenericMethod(t);
          return parseMethod.Invoke(null, [str.value]);
        }
      }
      throw new ApplicationException($"Cannot cast string to {t}");
    }

    // Bytes
    else if (duperValue is DuperValue.Bytes bytes)
    {
      if (typeof(byte[]).IsAssignableTo(t))
      {
        return bytes.value;
      }
      throw new ApplicationException($"Cannot cast bytes to {t}");
    }

    // Temporal
    else if (duperValue is DuperValue.Temporal temporal)
    {
      if (typeof(string).IsAssignableTo(t))
      {
        return temporal.value;
      }
      else if (typeof(DateOnly).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        // TO-DO: Proper conversion from Temporal value
        return DateOnly.Parse(temporal.value);
      }
      else if (typeof(TimeOnly).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        // TO-DO: Proper conversion from Temporal value
        return TimeOnly.Parse(temporal.value);
      }
      else if (typeof(DateTime).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        // TO-DO: Proper conversion from Temporal value
        return DateTime.Parse(temporal.value);
      }
      else if (typeof(DateTimeOffset).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        // TO-DO: Proper conversion from Temporal value
        return DateTimeOffset.Parse(temporal.value);
      }
      throw new ApplicationException($"Cannot cast temporal to {t}");
    }

    // Integer
    else if (duperValue is DuperValue.Integer integer)
    {
      if (typeof(long).IsAssignableTo(t))
      {
        return integer.value;
      }
      else if (typeof(int).IsAssignableTo(t))
      {
        return (int)integer.value;
      }
      else if (typeof(double).IsAssignableTo(t))
      {
        return (double)integer.value;
      }
      else if (typeof(float).IsAssignableTo(t))
      {
        return (float)integer.value;
      }
      throw new ApplicationException($"Cannot cast integer to {t}");
    }

    // Float
    else if (duperValue is DuperValue.Float flt)
    {
      if (typeof(double).IsAssignableTo(t))
      {
        return flt.value;
      }
      else if (typeof(float).IsAssignableTo(t))
      {
        return (float)flt.value;
      }
      throw new ApplicationException($"Cannot cast float to {t}");
    }

    // Boolean
    else if (duperValue is DuperValue.Boolean boolean)
    {
      if (typeof(bool).IsAssignableTo(t))
      {
        return boolean.value;
      }
      throw new ApplicationException($"Cannot cast boolean to {t}");
    }

    // Fail-safe
    else
    {
      throw new ApplicationException($"Unknown Duper value type {duperValue.GetType()}");
    }
  }

  private static T ParseViaGeneric<T>(string value) where T : IParsable<T>
  {
    return T.Parse(value, System.Globalization.CultureInfo.InvariantCulture);
  }

  public record SerializerOptions(
      string? Indent,
      bool StripIdentifiers,
      bool Minify
  )
  { }

  public static string Serialize<T>(T @value)
  {
    Type t = typeof(T);
    string? identifier = null;
    Attribute[] attrs = Attribute.GetCustomAttributes(t);
    foreach (Attribute attr in attrs)
    {
      if (attr is DuperAttribute a)
      {
        identifier = a.Identifier;
        break;
      }
    }
    var duperValue = SerializeInner(value, t, identifier);
    return Duper.Serialize(duperValue, null);
  }

  public static string Serialize<T>(T? @value, SerializerOptions @options)
  {
    Type t = typeof(T);
    string? identifier = null;
    Attribute[] attrs = Attribute.GetCustomAttributes(t);
    foreach (Attribute attr in attrs)
    {
      if (attr is DuperAttribute a)
      {
        identifier = a.Identifier;
        break;
      }
    }
    var duperValue = SerializeInner(value, t, identifier);
    return Duper.Serialize(duperValue, new Ffi.SerializeOptions(options.Indent, options.StripIdentifiers, options.Minify));
  }

  private static DuperValue SerializeInner(object? @value, Type t, string? identifier)
  {
    if (value == null)
    {
      return new DuperValue.Null(identifier);
    }

    t = Nullable.GetUnderlyingType(t) ?? t;
    if (t.IsAssignableTo(typeof(bool)))
    {
      return new DuperValue.Boolean(identifier, (bool)value);
    }
    else if (t.IsAssignableTo(typeof(double)))
    {
      return new DuperValue.Float(identifier, (double)value);
    }
    else if (t.IsAssignableTo(typeof(float)))
    {
      return new DuperValue.Float(identifier, (float)value);
    }
    else if (t.IsAssignableTo(typeof(long)))
    {
      return new DuperValue.Integer(identifier, (long)value);
    }
    else if (t.IsAssignableTo(typeof(int)))
    {
      return new DuperValue.Integer(identifier, (int)value);
    }
    else if (t.IsAssignableTo(typeof(byte[])))
    {
      return new DuperValue.Bytes(identifier, (byte[])value);
    }
    else if (t.IsAssignableTo(typeof(string)))
    {
      return new DuperValue.String(identifier, (string)value);
    }
    else if (t.IsAssignableTo(typeof(DateTimeOffset)))
    {
      if (identifier == "PlainDateTime")
      {
        return new DuperValue.Temporal(identifier, ((DateTimeOffset)value).DateTime.ToString("o", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainDate")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime(((DateTimeOffset)value).DateTime).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainTime")
      {
        return new DuperValue.Temporal(identifier, TimeOnly.FromDateTime(((DateTimeOffset)value).DateTime).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainYearMonth")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime(((DateTimeOffset)value).DateTime).ToString("yyyy-MM", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainMonthDay")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime(((DateTimeOffset)value).DateTime).ToString("MM-dd", System.Globalization.CultureInfo.InvariantCulture));
      }
      return new DuperValue.Temporal(identifier ?? "Instant", ((DateTimeOffset)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
    }
    else if (t.IsAssignableTo(typeof(DateTime)))
    {
      if (identifier == "PlainDate")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime((DateTime)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainTime")
      {
        return new DuperValue.Temporal(identifier, TimeOnly.FromDateTime((DateTime)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainYearMonth")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime((DateTime)value).ToString("yyyy-MM", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainMonthDay")
      {
        return new DuperValue.Temporal(identifier, DateOnly.FromDateTime((DateTime)value).ToString("MM-dd", System.Globalization.CultureInfo.InvariantCulture));
      }
      return new DuperValue.Temporal(identifier ?? "PlainDateTime", ((DateTime)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
    }
    else if (t.IsAssignableTo(typeof(DateOnly)))
    {
      if (identifier == "PlainYearMonth")
      {
        return new DuperValue.Temporal(identifier, ((DateOnly)value).ToString("yyyy-MM", System.Globalization.CultureInfo.InvariantCulture));
      }
      else if (identifier == "PlainMonthDay")
      {
        return new DuperValue.Temporal(identifier, ((DateOnly)value).ToString("MM-dd", System.Globalization.CultureInfo.InvariantCulture));
      }
      return new DuperValue.Temporal(identifier ?? "PlainDate", ((DateOnly)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
    }
    else if (t.IsAssignableTo(typeof(TimeOnly)))
    {
      return new DuperValue.Temporal(identifier ?? "PlainTime", ((TimeOnly)value).ToString("o", System.Globalization.CultureInfo.InvariantCulture));
    }
    else if (IsTuple(t))
    {
      var tupleFields = t.GetFields();
      DuperValue[] tupleValue = new DuperValue[tupleFields.Length];
      for (int i = 0; i < tupleFields.Length; i++)
      {
        var field = tupleFields[i];
        // TO-DO: Tuple identifiers
        tupleValue[i] = SerializeInner(field.GetValue(value), field.FieldType, null);
      }
      return new DuperValue.Tuple(identifier, tupleValue);
    }
    else if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IList<>))
    {
      Type itemType = t.GetGenericArguments().Single();
      IList valueList = (value as IList) ?? throw new ApplicationException("IList cast shouldn't fail");
      DuperValue[] arrayValue = new DuperValue[valueList.Count];
      for (int i = 0; i < valueList.Count; i++)
      {
        arrayValue[i] = SerializeInner(valueList[i], itemType, null);
      }
      return new DuperValue.Array(identifier, arrayValue);
    }
    else if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IDictionary<,>))
    {
      var generics = t.GetGenericArguments();
      Type keyType = generics[0];
      if (keyType != typeof(string))
      {
        throw new ApplicationException($"Cannot serialize dictionary with non-string keys to Duper");
      }
      Type valueType = generics[1];
      IDictionary valueDict = (value as IDictionary) ?? throw new ApplicationException("IDictionary cast shouldn't fail");
      List<DuperObjectEntry> objValue = new(valueDict.Count);
      foreach (var key in valueDict.Keys)
      {
        objValue.Add(new DuperObjectEntry((string)key, SerializeInner(valueDict[key], valueType, null)));
      }
      return new DuperValue.Object(identifier, [.. objValue]);
    }
    else if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IEnumerable<>))
    {
      Type itemType = t.GetGenericArguments().Single();
      IEnumerable valueEnumerable = (value as IEnumerable) ?? throw new ApplicationException("IEnumerable cast shouldn't fail");
      List<DuperValue> arrayValue = [];
      foreach (var element in valueEnumerable)
      {
        arrayValue.Add(SerializeInner(element, itemType, null));
      }
      return new DuperValue.Array(identifier, [.. arrayValue]);
    }

    Type? iformattable = null;
    foreach (Type interfaceType in t.GetInterfaces())
    {
      if (interfaceType.IsGenericType &&
          interfaceType.GetGenericTypeDefinition()
          == typeof(IList<>))
      {
        Type itemType = interfaceType.GetGenericArguments().Single();
        IList valueList = (value as IList) ?? throw new ApplicationException("IList cast shouldn't fail");
        DuperValue[] arrayValue = new DuperValue[valueList.Count];
        for (int i = 0; i < valueList.Count; i++)
        {
          arrayValue[i] = SerializeInner(valueList[i], itemType, null);
        }
        return new DuperValue.Array(identifier, arrayValue);
      }
      else if (interfaceType.IsGenericType &&
          interfaceType.GetGenericTypeDefinition()
          == typeof(IDictionary<,>))
      {
        var generics = interfaceType.GetGenericArguments();
        Type keyType = generics[0];
        if (keyType != typeof(string))
        {
          throw new ApplicationException($"Cannot serialize dictionary with non-string keys to Duper");
        }
        Type valueType = generics[1];
        IDictionary valueDict = (value as IDictionary) ?? throw new ApplicationException("IDictionary cast shouldn't fail");
        List<DuperObjectEntry> objValue = new(valueDict.Count);
        foreach (var key in valueDict.Keys)
        {
          objValue.Add(new DuperObjectEntry((string)key, SerializeInner(valueDict[key], valueType, null)));
        }
        return new DuperValue.Object(identifier, [.. objValue]);
      }
      else if (interfaceType.IsGenericType &&
          interfaceType.GetGenericTypeDefinition()
          == typeof(IEnumerable<>))
      {
        Type itemType = t.GetGenericArguments().Single();
        IEnumerable valueEnumerable = (value as IEnumerable) ?? throw new ApplicationException("IEnumerable cast shouldn't fail");
        List<DuperValue> arrayValue = [];
        foreach (var element in valueEnumerable)
        {
          arrayValue.Add(SerializeInner(element, itemType, null));
        }
        return new DuperValue.Array(identifier, [.. arrayValue]);
      }
      else if (interfaceType == typeof(IFormattable))
      {
        iformattable = interfaceType;
      }
    }

    if (iformattable != null)
    {
      var parseMethod = typeof(DuperSerializer).GetMethod("FormatViaGeneric", BindingFlags.NonPublic | BindingFlags.Static) ?? throw new ApplicationException("No FormatViaGeneric method found for DuperSerializer");
      parseMethod = parseMethod.MakeGenericMethod(t);
      var toStringResult = parseMethod.Invoke(null, [value]);
      if (toStringResult is string str)
      {
        return new DuperValue.String(identifier, str);
      }
    }

    List<DuperObjectEntry> classDict = [];
    Dictionary<string, DuperAttribute> duperAttributes = [];

    // Records: Check for Duper attribute in constructor parameters
    foreach (var constructor in t.GetConstructors())
    {
      foreach (var parameter in constructor.GetParameters())
      {
        string? name = parameter.Name;
        if (name == null)
        {
          continue;
        }
        Attribute[] fieldAttrs = Attribute.GetCustomAttributes(parameter);
        foreach (Attribute attr in fieldAttrs)
        {
          if (attr is DuperAttribute a)
          {
            duperAttributes.Add(name, a);
            break;
          }
        }
      }
    }

    foreach (var field in t.GetFields())
    {
      string key = field.Name;
      string? fieldIdentifier = null;
      duperAttributes.TryGetValue(field.Name, out DuperAttribute? duperAttribute);
      if (duperAttribute != null)
      {
        fieldIdentifier = duperAttribute.Identifier;
        if (duperAttribute.Key != null)
        {
          key = duperAttribute.Key;
        }
      }
      Attribute[] fieldAttrs = Attribute.GetCustomAttributes(field);
      foreach (Attribute attr in fieldAttrs)
      {
        if (attr is DuperAttribute a)
        {
          fieldIdentifier = a.Identifier;
          if (a.Key != null)
          {
            key = a.Key;
          }
          break;
        }
      }
      classDict.Add(new DuperObjectEntry(key, SerializeInner(field.GetValue(value), field.FieldType, fieldIdentifier)));
    }

    foreach (var prop in t.GetProperties())
    {
      string key = prop.Name;
      string? propIdentifier = null;
      duperAttributes.TryGetValue(prop.Name, out DuperAttribute? duperAttribute);
      if (duperAttribute != null)
      {
        propIdentifier = duperAttribute.Identifier;
        if (duperAttribute.Key != null)
        {
          key = duperAttribute.Key;
        }
      }
      Attribute[] fieldAttrs = Attribute.GetCustomAttributes(prop);
      foreach (Attribute attr in fieldAttrs)
      {
        if (attr is DuperAttribute a)
        {
          propIdentifier = a.Identifier;
          if (a.Key != null)
          {
            key = a.Key;
          }
          break;
        }
      }
      classDict.Add(new DuperObjectEntry(key, SerializeInner(prop.GetValue(value), prop.PropertyType, propIdentifier)));
    }

    return new DuperValue.Object(identifier, [.. classDict]);
  }

  private static string FormatViaGeneric<T>(T value) where T : IFormattable
  {
    return ((IFormattable)value).ToString(null, System.Globalization.CultureInfo.InvariantCulture);
  }
}
