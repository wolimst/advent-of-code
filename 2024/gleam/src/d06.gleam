import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/set.{type Set}
import gleam/string

type Guard {
  Guard(pos: #(Int, Int), dir: #(Int, Int))
}

fn parse(input: String) -> Dict(#(Int, Int), String) {
  let lines =
    input
    |> string.trim()
    |> string.split("\n")
    |> list.map(string.trim)

  let grid =
    lines
    |> list.index_map(fn(line, i) {
      line
      |> string.to_graphemes()
      |> list.index_map(fn(char, j) { #(#(i, j), char) })
    })
    |> list.flatten()
    |> dict.from_list()

  grid
}

fn get_guard(grid: Dict(#(Int, Int), String)) -> Guard {
  let guard = {
    use pos <-
      grid
      |> dict.filter(fn(_key, value) {
        list.contains(["^", "v", "<", ">"], value)
      })
      |> dict.keys()
      |> list.first()
      |> fn(res) { result.try(res, _) }

    use char <-
      dict.get(grid, pos)
      |> fn(res) { result.map(res, _) }

    case pos, char {
      pos, "^" -> Guard(pos, #(-1, 0))
      pos, ">" -> Guard(pos, #(0, 1))
      pos, "v" -> Guard(pos, #(1, 0))
      pos, "<" -> Guard(pos, #(0, -1))
      _, _ -> panic as "cannot reach here"
    }
  }

  case guard {
    Ok(guard) -> guard
    Error(_) -> panic as "cannot find guard"
  }
}

fn rotate_clockwise(dir: #(Int, Int)) -> #(Int, Int) {
  case dir {
    #(0, -1) -> #(-1, 0)
    #(-1, 0) -> #(0, 1)
    #(0, 1) -> #(1, 0)
    #(1, 0) -> #(0, -1)
    _ -> panic as "cannot reach here"
  }
}

fn get_next_dir(grid: Dict(#(Int, Int), String), guard: Guard) -> #(Int, Int) {
  let Guard(pos, dir) = guard

  let next_pos = #(pos.0 + dir.0, pos.1 + dir.1)
  case dict.get(grid, next_pos) {
    Ok("#") -> {
      let next_dir = rotate_clockwise(dir)
      get_next_dir(grid, Guard(pos, next_dir))
    }
    _ -> dir
  }
}

fn get_next_guard(grid: Dict(#(Int, Int), String), guard: Guard) -> Guard {
  let Guard(pos, _dir) = guard
  let next_dir = get_next_dir(grid, guard)
  let next_pos = #(pos.0 + next_dir.0, pos.1 + next_dir.1)
  Guard(next_pos, next_dir)
}

fn find_visits(
  grid: Dict(#(Int, Int), String),
  guard: Guard,
  visited: Set(#(Int, Int)),
) -> Set(#(Int, Int)) {
  case dict.get(grid, guard.pos) {
    Error(_) -> visited
    Ok("#") -> panic as "cannot reach here"
    Ok(_) -> {
      let visited = set.insert(visited, guard.pos)
      let next_guard = get_next_guard(grid, guard)
      find_visits(grid, next_guard, visited)
    }
  }
}

pub fn part1(input: String) -> Int {
  let grid = parse(input)
  let guard = get_guard(grid)

  let visited = find_visits(grid, guard, set.new())
  set.size(visited)
}

fn find_loop(grid: Dict(#(Int, Int), String), guard: Guard, visited: Set(Guard)) {
  case dict.get(grid, guard.pos) {
    Error(_) -> False
    Ok("#") -> panic as "cannot reach here"
    Ok(_) -> {
      let loop_found = case set.contains(visited, guard) {
        True -> True
        False -> {
          let visited = set.insert(visited, guard)
          let next_guard = get_next_guard(grid, guard)
          find_loop(grid, next_guard, visited)
        }
      }
      loop_found
    }
  }
}

pub fn part2(input: String) -> Int {
  let grid = parse(input)
  let guard = get_guard(grid)

  let len_i =
    grid |> dict.keys() |> list.map(fn(p) { p.0 }) |> list.max(int.compare)
  let len_j =
    grid |> dict.keys() |> list.map(fn(p) { p.1 }) |> list.max(int.compare)
  let #(len_i, len_j) = case len_i, len_j {
    Ok(i), Ok(j) -> #(i, j)
    _, _ -> panic as "cannot get grid size"
  }

  let coords =
    list.range(0, len_i)
    |> list.map(fn(i) {
      list.range(0, len_j)
      |> list.map(fn(j) { #(i, j) })
    })
    |> list.flatten()

  coords
  |> list.count(fn(coord) {
    let grid =
      dict.upsert(grid, coord, fn(value) {
        case value {
          Some(v) if v == "." -> "#"
          Some(v) -> v
          None -> panic as "cannot reach here"
        }
      })
    find_loop(grid, guard, set.new())
  })
}
