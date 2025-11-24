# Duper: The format that's super!

![GitHub license](https://img.shields.io/github/license/EpicEric/duper)

.NET bindings for Duper.

[Check out the official website for Duper.](https://duper.dev.br)

## C# example

```csharp
using System.Net;
using Duper;

namespace Example
{
  [Duper("UserProfile")]
  public class UserProfile
  {
    [Duper("Uuid")]
    public required string @id; // Support for public fields, too
    public required string @username { get; set; }
    [Duper("EmailAddress")]
    public required string @email { get; set; }
    public required UserSettings @settings { get; set; }
    public float @score { get; set; }
    [Duper("Png")]
    public required byte[] @avatar { get; set; }
    public string? @bio { get; set; }
    [Duper(Key = "last_logins")]
    public required IList<(IPAddress, DateTimeOffset)> LastLogins { get; set; }
  }

  public class UserSettings
  {
    [Duper(Key = "dark mode")]
    public bool DarkMode { get; set; }
    [Duper("Locale")]
    public required string @language { get; set; }
    public Dictionary<string, string?>? @metadata { get; set; }
  }

  public class Example
  {
    public static void Main(string[] args)
    {
      UserProfile userProfile = DuperSerializer.Deserialize<UserProfile>("""
        UserProfile({
          id: Uuid("f111c275-b4ce-4392-8e5b-19067ce39b53"),
          username: "EpicEric",
          email: EmailAddress("eric@duper.dev.br"),
          settings: {
            "dark mode": true,
            language: Locale("pt-BR"),
            metadata: null,
          },
          score: 120.25,
          // Support for bytes, woohoo!
          avatar: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ"),
          bio: r#"Hello! I'm a super "duper" user!"#,
          last_logins: [
            (IPv4Address("192.168.1.100"), Instant('2024-03-20T14:30:00+00:00')),
          ],
        })
        """) ?? throw new ApplicationException("shouldn't be null");

      Console.WriteLine(userProfile.settings.DarkMode);
      Console.WriteLine(userProfile.LastLogins[0].Item2);

      Console.WriteLine(DuperSerializer.Serialize(userProfile));
    }
  }
}
```
