defmodule D02 do
  @spec parse_input(String.t()) :: [[integer()]]
  def parse_input(str) do
    str
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split/1)
    |> Enum.map(&Enum.map(&1, fn x -> String.to_integer(x) end))
  end

  @spec safe?([integer()]) :: boolean()
  defp safe?([a, b | _rest] = list) when a < b, do: safe?(list, :asc)
  defp safe?([a, b | _rest] = list) when a > b, do: safe?(list, :desc)
  defp safe?([a, a | _rest]), do: false

  defp safe?([a, b | rest], :asc) do
    a < b && (b - a) in 1..3 && safe?([b | rest], :asc)
  end

  defp safe?([a, b | rest], :desc) do
    a > b && (a - b) in 1..3 && safe?([b | rest], :desc)
  end

  defp safe?([_last], _), do: true

  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> parse_input()
    |> Enum.count(&safe?/1)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input
    |> parse_input()
    |> Enum.count(fn list ->
      safe?(list) ||
        Enum.any?(0..(length(list) - 1), fn i ->
          deleted = List.delete_at(list, i)
          safe?(deleted)
        end)
    end)
  end
end
