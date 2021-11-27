namespace CubeShuffle.Core.Utility;

public static class ListExtensions
{
    public static void ShuffleMutable<T>(this IList<T> source, Random random) {
        var n = source.Count;
        for (var i = 0; i < n - 1; i++)
        {
            var r = i + random.Next(n - i);
            (source[r], source[i]) = (source[i], source[r]);
        }
    }
}