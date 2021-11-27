using CommandLine;

namespace CubeShuffle.CLI;

[Verb("distShuffle", true, HelpText = "Distribution based shuffle", Hidden = false)]
public class DistributionShuffleOptions
{
    private const string SourceGroup = "pileSource";
    
    [Option('n', "instructions", Default = false, Required = false, HelpText = "Gives instructions of how to use the shuffling method.")]
    public bool Instructions { get; set; }
    
    [Option('j', "piles", Default = null, Required = false, HelpText = "Explicitly defined card piles .", Group = SourceGroup)]
    public string? PilesJson { get; set; }
    
    [Option('i', "inputPiles", Default = null, Required = false, HelpText = "Import card piles from json.", Group = SourceGroup)]
    public string? PilesJsonPath { get; set; }

    [Option('s', "packSize", Default = 15, HelpText = "Size of each pack.")]
    public int PackSize { get; set; }

    [Option('o', "output", Default = OutputFormat.JSON, HelpText = "Output format.")]
    public OutputFormat Format { get; set; }
    
    [Option('r', "seed", Default = null, HelpText = "Seed for randomness.")]
    public int? Seed { get; set; }

    public enum OutputFormat
    {
        JSON
    }
}