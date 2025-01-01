defmodule Mix.Tasks.Day do
  use Mix.Task

  def usage() do
    IO.puts("Usage: mix day <number>")
    System.stop(1)
  end

  def solve(day) do
    IO.puts("Day #{day}")

    day = Integer.to_string(day) |> String.pad_leading(2, "0")

    input =
      Path.join([File.cwd!(), "..", "data", "d#{day}.txt"])
      |> File.read!()

    answer1 = apply(String.to_existing_atom("Elixir.D#{day}"), :part1, [input])
    IO.puts("Part 1: #{answer1}")
    answer2 = apply(String.to_existing_atom("Elixir.D#{day}"), :part2, [input])
    IO.puts("Part 2: #{answer2}")
  end

  @shortdoc "Usage: mix day <number>"
  def run(args) do
    if length(args) != 1, do: usage()

    case Integer.parse(hd(args)) do
      :error -> IO.puts("Invalid day number")
      {day, ""} -> solve(day)
      _ -> usage()
    end
  end
end
