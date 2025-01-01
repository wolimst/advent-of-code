defmodule D03Test do
  use ExUnit.Case

  test "solves part 1 example input" do
    input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    assert D03.part1(input) == 161
  end

  test "solves part 2 example input" do
    input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    assert D03.part2(input) == 48
  end
end
