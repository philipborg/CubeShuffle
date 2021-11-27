using System.Collections.Immutable;
using System.Globalization;
using System.Text;
using System.Text.Json;
using CommandLine;
using CubeShuffle.Core;

namespace CubeShuffle.CLI;

public static class Program
{
    public static void Main(string[] args)
    {
        using var parser = new Parser(settings =>
        {
            settings.AutoHelp = true;
            settings.AutoVersion = false;
            settings.CaseSensitive = true;
            settings.ParsingCulture = CultureInfo.InvariantCulture;
            settings.CaseInsensitiveEnumValues = false;
            settings.IgnoreUnknownArguments = false;
            settings.HelpWriter = Console.Out;
        });

        var jsonOptions = new JsonSerializerOptions(JsonSerializerDefaults.Web)
        {
            WriteIndented = true,
            DictionaryKeyPolicy = JsonNamingPolicy.CamelCase
        };

        parser.ParseArguments<DistributionShuffleOptions>(args)
            .WithParsed(options =>
            {
                var jsonPiles =
                    options.PilesJson
                    ?? (options.PilesJsonPath != null ? File.ReadAllText(options.PilesJsonPath, Encoding.UTF8) : null)
                    ?? throw new ArgumentException("Define a pile data source.");
                var piles = JsonSerializer.Deserialize<IImmutableDictionary<string, Pile>>(jsonPiles, jsonOptions)!;
                var randomSource = options.Seed != null ? new Random(options.Seed.Value) : new Random();
                var shuffled =
                    DistributionShuffle.DShuffle(piles, options.PackSize, randomSource)
                        .Select((pack, i) => new KeyValuePair<int,Pack>(i + 1, pack))
                        .ToImmutableSortedDictionary();
                
                var textOutput = options.Format switch
                {
                    DistributionShuffleOptions.OutputFormat.JSON => JsonSerializer.Serialize(shuffled, jsonOptions),
                    _ => throw new ArgumentOutOfRangeException(nameof(options.Format))
                };
                
                Console.WriteLine(textOutput);
            });
    }
}