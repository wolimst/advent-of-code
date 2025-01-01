defmodule D02Test do
  use ExUnit.Case

  test "solves part 1 example input" do
    input = """
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    """

    assert D02.part1(input) == 2
  end

  test "solves part 2 example input" do
    input = """
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    """

    assert D02.part2(input) == 4
  end
end
