using System.Net;

namespace Duper.Tests;

public class DuperSerializerTests
{
  [Fact]
  public void DuperSerializer_Null()
  {
    object? output = DuperSerializer.Deserialize<object?>("null");
    Assert.Null(output);

    Assert.Equal("null", DuperSerializer.Serialize<object?>(null));
  }

  [Fact]
  public void DuperSerializer_Boolean()
  {
    bool output = DuperSerializer.Deserialize<bool>("true");
    Assert.True(output);

    Assert.Equal("true", DuperSerializer.Serialize(true));
  }

  [Fact]
  public void DuperSerializer_Integer()
  {
    int output = DuperSerializer.Deserialize<int>("10");
    Assert.Equal(10, output);
    long output2 = DuperSerializer.Deserialize<long>("12345678910");
    Assert.Equal(12345678910, output2);

    Assert.Equal("10", DuperSerializer.Serialize<int>(10));
    Assert.Equal("12345678910", DuperSerializer.Serialize<long>(12345678910));
  }

  [Fact]
  public void DuperSerializer_Float()
  {
    float output = DuperSerializer.Deserialize<float>("10.2");
    Assert.Equal(10.2, output, 0.0001);
    double output2 = DuperSerializer.Deserialize<double>("2.7e+100");
    Assert.Equal(2.7e+100, output2);
  }


  [Fact]
  public void DuperSerializer_String()
  {
    string? output = DuperSerializer.Deserialize<string>("r\"super duper\"");
    Assert.Equal("super duper", output);
    IPAddress? output2 = DuperSerializer.Deserialize<IPAddress>("\"2001:12ff::1\"");
    Assert.Equal(IPAddress.Parse("2001:12ff::1"), output2);

    Assert.Equal("\"super duper\"", DuperSerializer.Serialize("super duper"));
    Assert.Equal("\"2001:12ff::1\"", DuperSerializer.Serialize(IPAddress.Parse("2001:12ff::1")));
  }

  [Fact]
  public void DuperSerializer_Temporal()
  {
    string? output = DuperSerializer.Deserialize<string>("'--12-25'");
    Assert.Equal("--12-25", output);
    DateOnly? output2 = DuperSerializer.Deserialize<DateOnly>("'1970-01-01'");
    Assert.Equal(new DateOnly(1970, 1, 1), output2);
    TimeOnly? output3 = DuperSerializer.Deserialize<TimeOnly>("'12:04:20'");
    Assert.Equal(new TimeOnly(12, 4, 20), output3);
    DateTime? output4 = DuperSerializer.Deserialize<DateTime>("'1970-01-01T12:04:20'");
    Assert.Equal(new DateTime(new DateOnly(1970, 1, 1), new TimeOnly(12, 4, 20), DateTimeKind.Unspecified), output4);
    DateTimeOffset? output5 = DuperSerializer.Deserialize<DateTime>("'1970-01-01T12:04:20+00:00'");
    Assert.Equal(new DateTimeOffset(new DateOnly(1970, 1, 1), new TimeOnly(12, 4, 20), TimeSpan.Zero), output5);

    Assert.Equal("PlainDate('1970-01-01')", DuperSerializer.Serialize(new DateOnly(1970, 1, 1)));
    Assert.Equal("PlainTime('12:04:20.0000000')", DuperSerializer.Serialize(new TimeOnly(12, 4, 20)));
    Assert.Equal("PlainDateTime('1970-01-01T12:04:20.0000000')", DuperSerializer.Serialize(new DateTime(new DateOnly(1970, 1, 1), new TimeOnly(12, 4, 20), DateTimeKind.Unspecified)));
    Assert.Equal("Instant('1970-01-01T12:04:20.0000000+00:00')", DuperSerializer.Serialize(new DateTimeOffset(new DateOnly(1970, 1, 1), new TimeOnly(12, 4, 20), TimeSpan.Zero)));
  }

  [Fact]
  public void DuperSerializer_Bytes()
  {
    byte[]? output = DuperSerializer.Deserialize<byte[]>(@"b""\x1b[0mabc""");
    Assert.Equal([27, 91, 48, 109, 97, 98, 99], output);

    Assert.Equal(@"b""\x1b[0mabc""", DuperSerializer.Serialize<byte[]>([27, 91, 48, 109, 97, 98, 99]));
  }

  [Fact]
  public void DuperSerializer_Tuple()
  {
    (string, object?) output = DuperSerializer.Deserialize<(string, object?)>(@"(""hello"", null)");
    Assert.Equal(("hello", null), output);
    int[]? output2 = DuperSerializer.Deserialize<int[]>("(12, 34)");
    Assert.Equal([12, 34], output2);
    IList<bool>? output3 = DuperSerializer.Deserialize<IList<bool>>("(true, false)");
    Assert.Equal([true, false], output3);
    List<byte[]?>? output4 = DuperSerializer.Deserialize<List<byte[]?>>(@"(b""a"", null)");
    Assert.Equal([[0x61], null], output4);

    Assert.Equal(@"(""hello"", null)", DuperSerializer.Serialize<(string, object?)>(("hello", null)));
  }

  [Fact]
  public void DuperSerializer_Array()
  {
    (string, object?) output = DuperSerializer.Deserialize<(string, object?)>(@"[""hello"", null]");
    Assert.Equal(("hello", null), output);
    int[]? output2 = DuperSerializer.Deserialize<int[]>(@"[12, 34]");
    Assert.Equal([12, 34], output2);
    IList<bool>? output3 = DuperSerializer.Deserialize<IList<bool>>(@"[true, false]");
    Assert.Equal([true, false], output3);
    List<byte[]?>? output4 = DuperSerializer.Deserialize<List<byte[]?>>(@"[b""a"", null]");
    Assert.Equal([[0x61], null], output4);

    Assert.Equal("[12, 34]", DuperSerializer.Serialize<int[]>([12, 34]));
    Assert.Equal("[true, false]", DuperSerializer.Serialize<IList<bool>>([true, false]));
    Assert.Equal(@"[b""a"", null]", DuperSerializer.Serialize<List<byte[]?>>([[0x61], null]));
  }

  [Fact]
  public void DuperSerializer_Object()
  {
    IDictionary<string, int?[]>? output = DuperSerializer.Deserialize<IDictionary<string, int?[]>>(@"{hello: [null, 14]}");
    Assert.Equivalent(new Dictionary<string, int?[]>() { { "hello", [null, 14] } }, output);
    Dictionary<string, (bool, string)>? output2 = DuperSerializer.Deserialize<Dictionary<string, (bool, string)>>(@"{""super duper"": (true, ""cool"")}");
    Assert.Equivalent(new Dictionary<string, (bool, string)>() { { "super duper", (true, "cool") } }, output2);

    Assert.Equal(@"{hello: [null, 14]}", DuperSerializer.Serialize(new Dictionary<string, int?[]>() { { "hello", [null, 14] } }));
    Assert.Equal(@"{""super duper"": (true, ""cool"")}", DuperSerializer.Serialize(new Dictionary<string, (bool, string)>() { { "super duper", (true, "cool") } }));
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
    public required IList<(IPAddress, DateTimeOffset)> LastLogins { get; set; }
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
  public void DuperSerializer_FullExample()
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

    string serialized = DuperSerializer.Serialize(userProfile);
    Assert.StartsWith("UserProfile({", serialized);
    Assert.EndsWith("})", serialized);
    Assert.Contains(@"id: Uuid(""f111c275-b4ce-4392-8e5b-19067ce39b53"")", serialized);
    Assert.Contains(@"username: ""EpicEric""", serialized);
    Assert.Contains(@"email: EmailAddress(""eric@duper.dev.br"")", serialized);
    Assert.Contains(@"settings: {", serialized);
    Assert.Contains(@"""dark mode"": true", serialized);
    Assert.Contains(@"language: Locale(""pt-BR"")", serialized);
    Assert.Contains(@"metadata: null", serialized);
    Assert.Contains(@"score: 120.25", serialized);
    Assert.Contains(@"avatar: Png(b64""iVBORw0KGgoAAAANSUhEUgAAAGQ="")", serialized);
    Assert.Contains(@"bio: r#""Hello! I'm a super ""duper"" user!""#", serialized);
    Assert.Contains(@"last_logins: [(""192.168.1.100"", Instant('2024-03-20T14:30:00.0000000+00:00'))]", serialized);
    Assert.Equal(388, serialized.Length);
  }
}
