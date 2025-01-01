defmodule D01Test do
  use ExUnit.Case

  test "solves part 1 example input" do
    input = """
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    """

    assert D01.part1(input) == 11
  end

  test "solves part 2 example input" do
    input = """
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    """

    assert D01.part2(input) == 31
  end
end
