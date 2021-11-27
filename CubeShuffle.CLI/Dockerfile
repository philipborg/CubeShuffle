FROM mcr.microsoft.com/dotnet/runtime:6.0-alpine AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:6.0-alpine AS build
WORKDIR /src
COPY ["CubeShuffle.Core/CubeShuffle.Core.csproj", "CubeShuffle.Core/"]
COPY ["CubeShuffle.CLI/CubeShuffle.CLI.csproj", "CubeShuffle.CLI/"]
RUN dotnet restore "CubeShuffle.CLI/CubeShuffle.CLI.csproj"
COPY . .
WORKDIR "/src/CubeShuffle.CLI"
RUN dotnet build "CubeShuffle.CLI.csproj" -c Release -o /app/build --no-restore

FROM build AS publish
RUN dotnet publish "CubeShuffle.CLI.csproj" -c Release -o /app/publish --no-build

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "CubeShuffle.CLI.dll"]
