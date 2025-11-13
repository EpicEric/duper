using Duper;

namespace Example
{
  [Duper("UserProfile")]
  public class UserProfile
  {
    [Duper("Uuid")]
    public required string @id;
    public required string @username;
    [Duper("EmailAddress")]
    public required string @email;
    public required UserSettings @settings;
    public double @score;
    [Duper("Png")]
    public required byte[] @avatar;
    public string? @bio;
    public required IList<(string, DateTimeOffset)> @last_logins;
  }

  public class UserSettings
  {
    [Duper(Key = "dark mode")]
    public bool DarkMode;
    [Duper("Locale")]
    public required string @language;
    public Dictionary<string, string>? @metadata;
  }

  public class Example
  {
    public static void Main(string[] args)
    {
      UserProfile? userProfile = DuperSerializer.Deserialize<UserProfile>("""
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
      Console.WriteLine(userProfile.last_logins[0].Item2);

      // Console.WriteLine(DuperSerializer.Serialize(userProfile));
    }
  }
}
