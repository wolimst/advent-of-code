defmodule D04 do
  @spec parse_input(String.t()) :: %{{integer(), integer()} => String.t()}
  def parse_input(str) do
    lines = String.split(str, "\n", trim: true)

    grid =
      for {row, i} <- Enum.with_index(lines),
          {char, j} <- Enum.with_index(String.split(row, "", trim: true)),
          into: %{},
          do: {{i, j}, char}

    grid
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    dirs =
      for di <- -1..1,
          dj <- -1..1,
          not (di == 0 and dj == 0),
          do: {di, dj}

    grid = parse_input(input)

    grid
    |> Map.filter(fn {_coord, char} -> char == "X" end)
    |> Map.keys()
    |> Enum.map(fn {i, j} ->
      dirs
      |> Enum.map(fn {di, dj} ->
        Stream.iterate({i, j}, fn {i, j} -> {i + di, j + dj} end)
        |> Enum.take(4)
        |> Enum.map(&grid[&1])
        |> Enum.join()
      end)
      |> Enum.count(&(&1 == "XMAS"))
    end)
    |> Enum.sum()
  end

  def part2(input) do
    dirs =
      for di <- [-1, 1],
          dj <- [-1, 1],
          do: {di, dj}

    grid = parse_input(input)

    grid
    |> Map.filter(fn {_coord, char} -> char == "A" end)
    |> Map.keys()
    |> Enum.map(fn {i, j} ->
      dirs
      |> Enum.map(fn {di, dj} ->
        [{i - di, j - dj}, {i, j}, {i + di, j + dj}]
        |> Enum.map(&grid[&1])
        |> Enum.join()
      end)
      |> Enum.count(&(&1 == "MAS"))
    end)
    |> Enum.count(&(&1 == 2))
  end
end
