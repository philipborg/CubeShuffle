namespace CubeShuffle.Core.Utility;

public static class RandomExtensions
{
    public static bool NextBool(this Random random) => random.Next(0, 1) == 1;

    public static bool NextBool(this Random random, double odds) =>
        odds switch
        {
            < 0d => throw new ArgumentOutOfRangeException(nameof(odds), "Odds is less than 0%."),
            > 1d => throw new ArgumentOutOfRangeException(nameof(odds), "Odds is over 100%."),
            _ => random.NextDouble() < odds
        };
}