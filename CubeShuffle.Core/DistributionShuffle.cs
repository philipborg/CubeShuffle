using System.Collections.Immutable;
using CubeShuffle.Core.Utility;

namespace CubeShuffle.Core;

public static class DistributionShuffle
{
    public static IEnumerable<Pack> DShuffle(IImmutableDictionary<string, Pile> piles, int packSize, Random random)
    {
        if (packSize == 0)
            throw new ArgumentOutOfRangeException(nameof(packSize), "Pack can't be of size zero.");
        
        var totalNumberOfCards = piles.Values.Sum(pile => pile.Cards);

        var (numberOfPacks, packOverflow) = Math.DivRem(totalNumberOfCards, packSize);
        if (packOverflow != 0)
        {
            throw new ArgumentOutOfRangeException(
                nameof(packSize),
                $"Total number of cards, {totalNumberOfCards}, isn't divisible by pack size {packSize}.");
        }

        var packs = Enumerable.Range(1, numberOfPacks)
            .Select(_ => new List<string?>())
            .ToImmutableArray();
        
        var randomized = new List<string>();
        foreach (var (pileName, pile) in piles.Shuffle(random))
        {
            for (var i = 0; i < pile.Cards; i++)
            {
                var skip = random.NextBool(pile.Randomness);
                if(skip)
                    randomized.Add(pileName);
                packs[i % numberOfPacks].Add(skip ? null : pileName);
            }

            packs = packs
                .GroupBy(l => l.Count)
                .OrderBy(group => group.Key)
                .SelectMany(group => group.Shuffle(random))
                .ToImmutableArray();
        }

        var randomFilling = randomized.Shuffle(random).ToStack();
        return packs
            .Select(pack =>
            {
                var cardSource = pack
                    .Select(pile => pile ?? randomFilling.Pop())
                    .GroupBy(pile => pile)
                    .ToImmutableSortedDictionary(
                        group => @group.Key,
                        group => @group.Count()
                    );
                return new Pack(cardSource);
            });
    }
}