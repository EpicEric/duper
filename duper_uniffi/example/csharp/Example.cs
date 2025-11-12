using Duper;

namespace Example
{
  public class UserProfile
  {
    public required string @id;
    public required string @username;
    public required string @email;
    public required UserSettings @settings;
    public double @score;
    public required byte[] @avatar;
    public string? @bio;
    public required IList<(string, string)> @last_logins;
  }

  public class UserSettings
  {
    public bool DarkMode;
    public required string @language;
    public Dictionary<string, string>? @metadata;
  }

  public class Example
  {
    public static void Main(string[] args)
    {
      UserProfile? userProfile = DuperSerializer.Deserialize<UserProfile>(@"
        UserProfile({
          id: Uuid(""f111c275-b4ce-4392-8e5b-19067ce39b53""),
          username: ""EpicEric"",
          email: EmailAddress(""eric@duper.dev.br""),
          settings: {
            ""dark mode"": true,
            language: Locale(""pt-BR""),
            metadata: null,
          },
          score: 120.25,
          // Support for bytes, woohoo!
          avatar: Png(b64""iVBORw0KGgoAAAANSUhEUgAAAGQ""),
          bio: r#""Hello! I'm a super ""duper"" user!""#,
          last_logins: [
            (IPv4Address(""192.168.1.100""), Instant('2024-03-20T14:30:00+00:00')),
          ],
        })");
      Console.WriteLine(userProfile);
      // Console.WriteLine(DuperSerializer.Serialize(userProfile));
    }
  }
}
