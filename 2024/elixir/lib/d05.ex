defmodule D05 do
  @spec parse_rules(String.t()) :: [{integer(), integer()}]
  def parse_rules(input) do
    input
    |> String.split("\n\n", trim: true)
    |> Enum.at(0)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [a, b] = String.split(line, "|", trim: true)
      {String.to_integer(a), String.to_integer(b)}
    end)
  end

  @spec parse_updates(String.t()) :: [[integer()]]
  def parse_updates(input) do
    input
    |> String.split("\n\n", trim: true)
    |> Enum.at(1)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> String.split(",", trim: true)
      |> Enum.map(&String.to_integer/1)
    end)
  end

  @spec valid?([integer()], {integer(), integer()}) :: boolean()
  def valid?(update, {left, right}) do
    left_index = Enum.find_index(update, &(&1 == left))
    right_index = Enum.find_index(update, &(&1 == right))

    case {left_index, right_index} do
      {nil, _} -> true
      {_, nil} -> true
      _ -> left_index < right_index
    end
  end

  @spec valid?([integer()], [{integer(), integer()}]) :: boolean()
  def valid?(update, rules) do
    rules
    |> Enum.all?(fn rule -> valid?(update, rule) end)
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    rules = parse_rules(input)
    updates = parse_updates(input)

    updates
    |> Enum.filter(fn update -> valid?(update, rules) end)
    |> Enum.map(&Enum.at(&1, div(length(&1), 2)))
    |> Enum.sum()
  end

  @spec reorder([integer()], [{integer(), integer()}]) :: [integer()]
  def reorder(update, rules) do
    {left, right} =
      rules
      |> Enum.filter(fn {left, right} ->
        Enum.member?(update, left) and Enum.member?(update, right)
      end)
      |> Enum.unzip()

    first = Enum.find(left, fn x -> x not in right end)

    rest =
      right
      |> Enum.frequencies()
      |> Enum.sort_by(fn {_k, v} -> v end)
      |> Enum.unzip()
      |> elem(0)

    [first | rest]
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    rules = parse_rules(input)
    updates = parse_updates(input)

    updates
    |> Enum.reject(fn update -> valid?(update, rules) end)
    |> Enum.map(fn update -> reorder(update, rules) end)
    |> Enum.map(&Enum.at(&1, div(length(&1), 2)))
    |> Enum.sum()
  end
end
