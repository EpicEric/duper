namespace Duper;

[AttributeUsage(AttributeTargets.Class | AttributeTargets.Struct | AttributeTargets.Field | AttributeTargets.Property | AttributeTargets.Parameter)]
public class DuperAttribute : Attribute
{
  public string? Identifier;
  public string? Key;

  public DuperAttribute(string? identifier)
  {
    Identifier = identifier;
    Key = null;
  }

  public DuperAttribute()
  {
    Identifier = null;
    Key = null;
  }
}
