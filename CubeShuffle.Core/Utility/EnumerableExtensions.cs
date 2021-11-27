using System.Diagnostics.Contracts;

namespace CubeShuffle.Core.Utility;

public static class EnumerableExtensions
{
    public static IEnumerable<T> Shuffle<T>(this IEnumerable<T> source, Random random)
    {
        var items = source.ToList();
        items.ShuffleMutable(random);
        return items;
    }

    [Pure]
    public static Queue<T> ToQueue<T>(this IEnumerable<T> source) => new(source);
    [Pure]
    public static Stack<T> ToStack<T>(this IEnumerable<T> source) => new(source);
}