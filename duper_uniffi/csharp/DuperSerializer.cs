namespace Duper;

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
    var t2 = t.GetGenericTypeDefinition();
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
    var duperValue = Duper.Parse(input, true);
    var t = typeof(T);
    var value = DeserializeInner(duperValue, typeof(T));
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
          addMethod.Invoke(dict, [item.Key, DeserializeInner(item.Value, valueType)]);
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
            addMethod.Invoke(dict, [item.Key, DeserializeInner(item.Value, valueType)]);
          }
          return dict;
        }
      }
      // Create class instance
      object instance = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
      Dictionary<string, System.Reflection.FieldInfo> fields = [];
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
        var item = obj.value[key] ?? throw new ApplicationException($"No key {key} found in Duper object");
        field.SetValue(instance, DeserializeInner(item, field.FieldType));
      }
      return instance;
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
      else if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IList<>))
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
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IList<>))
        {
          Type itemType = interfaceType.GetGenericArguments().Single();
          var list = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
          var addMethod = interfaceType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for IList");
          foreach (var item in array.value)
          {
            addMethod.Invoke(list, [DeserializeInner(item, itemType)]);
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
      else if (t.IsGenericType && t.GetGenericTypeDefinition() == typeof(IList<>))
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
      foreach (Type interfaceType in t.GetInterfaces())
      {
        if (interfaceType.IsGenericType &&
            interfaceType.GetGenericTypeDefinition()
            == typeof(IList<>))
        {
          Type itemType = interfaceType.GetGenericArguments().Single();
          var list = Activator.CreateInstance(t) ?? throw new ApplicationException($"No constructor found for {t}");
          var addMethod = interfaceType.GetMethod("Add") ?? throw new ApplicationException("No Add method found for IList");
          foreach (var item in tuple.value)
          {
            addMethod.Invoke(list, [DeserializeInner(item, itemType)]);
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
    else if (duperValue is DuperValue.Temporal temporal)
    {
      if (typeof(string).IsAssignableTo(t))
      {
        return temporal.value;
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
      throw new ApplicationException($"Unknown DuperValue type {duperValue.GetType()}");
    }
  }

  public record SerializerOptions(
      string? Indent,
      bool StripIdentifiers,
      bool Minify
  )
  { }

  public static string Serialize<T>(T @value)
  {
    var duperValue = SerializeInner<T>(value);
    return Duper.Serialize(duperValue, null);
  }

  public static string Serialize<T>(T? @value, SerializerOptions @options)
  {
    var duperValue = SerializeInner<T>(value);
    return Duper.Serialize(duperValue, new SerializeOptions(options.Indent, options.StripIdentifiers, options.Minify));
  }

  private static DuperValue SerializeInner<T>(T? @value)
  {
    var t = typeof(T);
    Attribute[] attrs = Attribute.GetCustomAttributes(t);
    string? identifier;
    foreach (Attribute attr in attrs)
    {
      if (attr is DuperAttribute a)
      {
        identifier = a.Identifier;
      }
    }
    throw new ApplicationException("Unimplemented");
  }
}
