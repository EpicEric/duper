namespace Duper;

public class DuperSerializer
{
  public static T? Deserialize<T>(string @input)
  {
    var parsed = Ffi.Duper.Parse(input, true);
    Console.WriteLine(parsed);
    Console.WriteLine(typeof(T));
    throw new ApplicationException("Unimplemented");
  }

  public static string Serialize<T>(T @value, bool minify)
  {
    throw new ApplicationException("Unimplemented");
  }
}
