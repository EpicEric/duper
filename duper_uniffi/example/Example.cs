using Duper;

class Example
{
  static void Main(string[] args)
  {
    var output = Duper.Duper.Parse(@"
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
      })
    ", false);
    var obj = (DuperValue.Object)output;
    var id = (DuperValue.String)obj.value["id"];
    Console.WriteLine(id.value);
  }
}
