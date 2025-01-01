defmodule D03 do
  @spec scan(String.t(), :disabled | :enabled) :: integer()
  def scan(str, :enabled) do
    case Regex.split(~r/don\'t\(\)/, str, parts: 2) do
      [valid, rest] -> part1(valid) + scan(rest, :disabled)
      [valid] -> part1(valid)
    end
  end

  def scan(str, :disabled) do
    case Regex.split(~r/do\(\)/, str, parts: 2) do
      [_invalid, rest] -> scan(rest, :enabled)
      [_invalid] -> 0
    end
  end

  @spec part1(String.t()) :: integer()
  def part1(str) do
    ~r/mul\((\d+),(\d+)\)/
    |> Regex.scan(str)
    |> Enum.map(fn [_matched_str, a, b] -> String.to_integer(a) * String.to_integer(b) end)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(str) do
    scan(str, :enabled)
  end
end
