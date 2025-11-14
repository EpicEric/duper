using System.Net;

namespace Duper.Tests;

public class DuperSerializerTests
{
  [Fact]
  public void DuperSerializer_Deserialize_Null()
  {
    object? output = DuperSerializer.Deserialize<object?>("null");
    Assert.Null(output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Boolean()
  {
    bool output = DuperSerializer.Deserialize<bool>("true");
    Assert.True(output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Int()
  {
    int output = DuperSerializer.Deserialize<int>("10");
    Assert.Equal(10, output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Long()
  {
    long output = DuperSerializer.Deserialize<long>("12345678910");
    Assert.Equal(12345678910, output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Float()
  {
    float output = DuperSerializer.Deserialize<float>("10.2");
    Assert.Equal(10.2, output, 0.0001);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Double()
  {
    double output = DuperSerializer.Deserialize<double>("2.7e+100");
    Assert.Equal(2.7e+100, output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_String()
  {
    string? output = DuperSerializer.Deserialize<string>(@"r""super duper""");
    Assert.Equal("super duper", output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Bytes()
  {
    byte[]? output = DuperSerializer.Deserialize<byte[]>(@"b""\x1b[0m""");
    Assert.Equal([27, 91, 48, 109], output);
  }

  [Fact]
  public void DuperSerializer_Deserialize_Tuple()
  {
    (string, object?) output = DuperSerializer.Deserialize<(string, object?)>(@"(""hello"", null)");
    Assert.Equal(("hello", null), output);
  }

  [Duper("UserProfile")]
  public class UserProfileExample
  {
    [Duper("Uuid")]
    public required string @id; // Support for public fields, too
    public required string @username { get; set; }
    [Duper("EmailAddress")]
    public required string @email { get; set; }
    public required UserSettingsExample @settings { get; set; }
    public float @score { get; set; }
    [Duper("Png")]
    public required byte[] @avatar { get; set; }
    public string? @bio { get; set; }
    [Duper(Key = "last_logins")]
    public required IList<(System.Net.IPAddress, DateTimeOffset)> LastLogins { get; set; }
  }

  public class UserSettingsExample
  {
    [Duper(Key = "dark mode")]
    public bool DarkMode { get; set; }
    [Duper("Locale")]
    public required string @language { get; set; }
    public Dictionary<string, string?>? @metadata { get; set; }
  }

  [Fact]
  public void DuperSerializer_Deserialize_FullExample()
  {
    UserProfileExample? userProfile = DuperSerializer.Deserialize<UserProfileExample>("""
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
      """);
    Assert.NotNull(userProfile);

    Assert.Equal("f111c275-b4ce-4392-8e5b-19067ce39b53", userProfile.id);
    Assert.Equal("EpicEric", userProfile.username);
    Assert.Equal("eric@duper.dev.br", userProfile.email);
    Assert.Equivalent(new UserSettingsExample { DarkMode = true, language = "pt-BR", metadata = null }, userProfile.settings);
    Assert.Equal(120.25, userProfile.score);
    Assert.Equal([137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 100], userProfile.avatar);
    Assert.Equal("Hello! I'm a super \"duper\" user!", userProfile.bio);
    Assert.Equal([(IPAddress.Parse("192.168.1.100"), DateTimeOffset.Parse("2024-03-20T14:30:00+00:00"))], userProfile.LastLogins);
  }
}
