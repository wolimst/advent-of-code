defmodule D01 do
  @spec parse_input(String.t()) :: {[integer()], [integer()]}
  def parse_input(str) do
    str
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split/1)
    |> Enum.map(fn [a, b] -> {String.to_integer(a), String.to_integer(b)} end)
    |> Enum.unzip()
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    {left, right} = parse_input(input)
    left_sorted = Enum.sort(left)
    right_sorted = Enum.sort(right)

    Enum.zip(left_sorted, right_sorted)
    |> Enum.map(fn {a, b} -> abs(a - b) end)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    {left, right} = parse_input(input)
    right_freq = Enum.frequencies(right)

    left
    |> Enum.map(fn x -> x * (right_freq[x] || 0) end)
    |> Enum.sum()
  end
end
