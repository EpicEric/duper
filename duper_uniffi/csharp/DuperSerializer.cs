namespace Duper;

using Ffi;

public class DuperSerializer
{
  public static T? Deserialize<T>(string @input)
  {
    var parsed = Duper.Parse(input, true);
    return DeserializeInner<T>(parsed);
  }

  private static T? DeserializeInner<T>(DuperValue duperValue)
  {
    var t = typeof(T);
    if (duperValue is DuperValue.Null)
    {
      if (!t.IsValueType || Nullable.GetUnderlyingType(t) != null)
      {
        // Returns null for reference types and nullable value types
        return default;
      }
      throw new ApplicationException($"Cannot cast null to non-nullable {t}");
    }
    else if (duperValue is DuperValue.Object obj)
    {
      throw new ApplicationException("Unimplemented");
    }
    else if (duperValue is DuperValue.Array array)
    {
      throw new ApplicationException("Unimplemented");
    }
    else if (duperValue is DuperValue.Tuple tuple)
    {
      throw new ApplicationException("Unimplemented");
    }
    else if (duperValue is DuperValue.String str)
    {
      if (typeof(string).IsAssignableTo(t))
      {
        return (T)(object)str.value;
      }
      throw new ApplicationException($"Cannot cast string to {t}");
    }
    else if (duperValue is DuperValue.Bytes bytes)
    {
      if (typeof(byte[]).IsAssignableTo(t))
      {
        return (T)(object)bytes.value;
      }
      throw new ApplicationException($"Cannot cast bytes to {t}");
    }
    else if (duperValue is DuperValue.Temporal temporal)
    {
      if (typeof(string).IsAssignableTo(t))
      {
        return (T)(object)temporal.value;
      }
      else if (typeof(DateTime).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        return (T)(object)DateTime.Parse(temporal.value);
      }
      else if (typeof(DateTimeOffset).IsAssignableTo(t))
      {
        // TO-DO: Validate the identifier first
        return (T)(object)DateTimeOffset.Parse(temporal.value);
      }
      throw new ApplicationException($"Cannot cast temporal to {t}");
    }
    else if (duperValue is DuperValue.Integer integer)
    {
      if (typeof(long).IsAssignableTo(t))
      {
        return (T)(object)integer.value;
      }
      else if (typeof(int).IsAssignableTo(t))
      {
        return (T)(object)(int)integer.value;
      }
      else if (typeof(double).IsAssignableTo(t))
      {
        return (T)(object)(double)integer.value;
      }
      else if (typeof(float).IsAssignableTo(t))
      {
        return (T)(object)(float)integer.value;
      }
      throw new ApplicationException($"Cannot cast integer to {t}");
    }
    else if (duperValue is DuperValue.Float flt)
    {
      if (typeof(double).IsAssignableTo(t))
      {
        return (T)(object)flt.value;
      }
      else if (typeof(float).IsAssignableTo(t))
      {
        return (T)(object)(float)flt.value;
      }
      throw new ApplicationException($"Cannot cast float to {t}");
    }
    else if (duperValue is DuperValue.Boolean boolean)
    {
      if (typeof(bool).IsAssignableTo(t))
      {
        return (T)(object)boolean.value;
      }
      throw new ApplicationException($"Cannot cast boolean to {t}");
    }
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
