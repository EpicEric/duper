namespace Duper;

public class DuperSerializeException : Exception
{
  internal DuperSerializeException(string message)
      : base(message) { }

  internal DuperSerializeException(string message, Exception exception)
      : base(message, exception) { }

  public class SerializeOptionsException : DuperSerializeException
  {
    internal SerializeOptionsException(string message)
        : base(message) { }

    internal SerializeOptionsException(string message, Exception exception)
        : base(message, exception) { }
  }

  public class InvalidIdentifierException : DuperSerializeException
  {
    internal InvalidIdentifierException(string message)
        : base(message) { }

    internal InvalidIdentifierException(string message, Exception exception)
        : base(message, exception) { }
  }

  public class InvalidObjectException : DuperSerializeException
  {
    internal InvalidObjectException(string message)
        : base(message) { }

    internal InvalidObjectException(string message, Exception exception)
        : base(message, exception) { }
  }

  public class InvalidTemporalException : DuperSerializeException
  {
    internal InvalidTemporalException(string message)
        : base(message) { }

    internal InvalidTemporalException(string message, Exception exception)
        : base(message, exception) { }
  }

  public class InvalidValueException : DuperDeserializeException
  {
    internal InvalidValueException(string message)
        : base(message) { }

    internal InvalidValueException(string message, Exception exception)
        : base(message, exception) { }
  }
}