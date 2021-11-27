using System.Collections.Immutable;

namespace CubeShuffle.Core;

public record Pack(IImmutableDictionary<string, int> CardSources);