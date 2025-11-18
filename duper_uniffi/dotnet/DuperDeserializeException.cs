namespace Duper;

public class DuperDeserializeException : Exception
{
  internal DuperDeserializeException(string message)
      : base(message) { }

  internal DuperDeserializeException(string message, Exception exception)
      : base(message, exception) { }

  public class ParseException : DuperDeserializeException
  {
    internal ParseException(string message)
        : base(message) { }

    internal ParseException(string message, Exception exception)
        : base(message, exception) { }
  }

  public class InvalidTypeException : DuperDeserializeException
  {
    internal InvalidTypeException(string message)
        : base(message) { }

    internal InvalidTypeException(string message, Exception exception)
        : base(message, exception) { }
  }
}