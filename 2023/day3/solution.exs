defmodule Solution do
  def part_one lines do
    matched_i = Enum.map(0..length(lines) - 1, fn row_i ->
      row = Enum.at(lines, row_i)
      Enum.filter(0..String.length(row) - 1, fn col_i ->
        curr_char = String.at(row, col_i)
        String.match?(curr_char, ~r/\d/) and is_adj_to_symbol(lines, row_i, col_i)
      end)
    end) |> Enum.with_index()
    IO.inspect(matched_i)
    sum = Enum.reduce(matched_i, 0, fn {row, row_i}, total_sum ->
      {_end_col, row_sum} = Enum.reduce(row, {nil, total_sum}, fn col_i, {prev, sum} ->
        if prev == nil or prev != col_i - 1 do
          {col_i, sum + get_number_at(lines, row_i, col_i)}
        else
          {col_i, sum}
        end
      end)
      row_sum
    end)
    IO.inspect(sum)
  end

  def part_two lines do

    parsed_matrix = Enum.map(lines, fn line ->
      split_line = String.split(line, "", trim: true)
      {lst, _i} = Enum.reduce(split_line, {[], 0}, fn x, {lst, col_i} ->
        {Enum.concat(lst, [{x, col_i}]), col_i + 1}
      end)
      lst
    end) |> Enum.with_index()
    parsed_matrix = Enum.map(parsed_matrix, fn {line, row_i} ->
      Enum.map(line, fn {char, col_i} ->
        cond do
          char == "*" -> char
          String.match?(char, ~r/\d/) -> get_start_end(lines, row_i, col_i)
          true -> nil
        end
      end)
    end)
    sum = Enum.with_index(parsed_matrix) |> Enum.reduce(0, fn { line, row_i}, acc ->
      s = Enum.with_index(line) |> Enum.reduce(0, fn {ele, col_i}, inner_acc ->
        inner_acc + mult_if_two(ele, parsed_matrix, row_i, col_i, lines)
      end)
      acc + s
    end)
    IO.inspect(sum)
  end

  def mult_if_two(ele, parsed_matrix, row_i, col_i, lines) do
    case ele do
      "*" -> mult_if_two(parsed_matrix, row_i, col_i, lines)
      _ -> 0
    end
  end

  def mult_if_two(parsed_matrix, row, col, lines) do
    max_row = length(lines)
    max_col = String.length(Enum.at(lines, 0))
    uniq_eles = Enum.map(max(0, row-1)..min(row+1, max_row-1), fn row_i ->
      Enum.map(max(0, col-1)..min(col+1, max_col-1), fn col_i ->
        {Enum.at(Enum.at(parsed_matrix, row_i), col_i), row_i}
      end)
    end) |> List.flatten()  |> Enum.filter( fn ele ->
      case ele do
       {{_x, _y}, _i} -> true
       _ -> false
      end
    end) |> Enum.uniq()
    # IO.inspect(uniq_eles)
    case uniq_eles do
      [{{start_i_1, end_i_1}, row_i_1}, {{start_i_2, end_i_2}, row_i_2}] ->
        String.to_integer(String.slice(Enum.at(lines, row_i_1), start_i_1..end_i_1)) *
        String.to_integer(String.slice(Enum.at(lines, row_i_2), start_i_2..end_i_2))
     _ -> 0
    end
  end

  def get_start_end lines, row_i, col_i do
    line = Enum.at(lines, row_i)
    max_col_i = String.length(line) - 1
    sub_string_start = cond do
      0 === col_i -> 0
      true -> Enum.find(col_i..1, 0, fn cur ->
        !String.match?(String.at(line, cur - 1), ~r/\d/)
      end)
    end
    sub_string_end = cond do
      col_i === max_col_i -> col_i
      true -> Enum.find(col_i..(max_col_i - 1), max_col_i, fn cur ->
        # IO.inspect("col_i: #{col_i}, cur: #{cur}, at cur+1: #{String.at(line, cur + 1)}")
        !String.match?(String.at(line, cur + 1), ~r/\d/)
    end)
    end
    {sub_string_start, sub_string_end}
  end

  def read_file do
    File.read!("./day3/input.txt")
    |> String.split("\n", trim: true)
  end

  @spec get_number_at(list(String.t()), integer(), integer()) :: integer()
  def get_number_at lines, row_i, col_i do
    line = Enum.at(lines, row_i)
    max_col_i = String.length(line) - 1
    sub_string_start = Enum.find(col_i..1, 0, fn cur ->
      !String.match?(String.at(line, cur - 1), ~r/\d/)
    end)
    sub_string_end = Enum.find(col_i..max_col_i - 1, max_col_i, fn cur ->
        !String.match?(String.at(line, cur + 1), ~r/\d/)
    end)
    IO.inspect("getting sub of " <> line <> " " <> to_string(sub_string_start) <> "," <> to_string(sub_string_end) <> " " <> String.slice(line, sub_string_start..sub_string_end))
    IO.inspect(String.slice(line, sub_string_start..sub_string_end))
    String.to_integer(String.slice(line, sub_string_start..sub_string_end))
  end

  def is_adj_to_symbol lines, row, col do
    max_row = length(lines)
    max_col = String.length(Enum.at(lines, 0))
    Enum.map(max(0, row-1)..min(row+1, max_row-1), fn row_i ->
      Enum.map(max(0, col-1)..min(col+1, max_col-1), fn col_i ->
        # IO.puts("row_i " <> to_string(row_i) <> " col_i " <> to_string(col_i) <> " char: " <> String.at(Enum.at(lines, row_i), col_i))
        String.at(Enum.at(lines, row_i), col_i)
      end)
    end) |> Enum.reduce(false, fn lst, acc ->
      acc or Enum.reduce(lst, false, fn str, acc ->
        # IO.puts("curr: " <> to_string(row) <> "," <> to_string(col) <> " check: " <> str <> " match?: " <> to_string((!String.match?(str, ~r/\d/) and str != ".")))
      (!String.match?(str, ~r/\d/) and str != ".") or acc
    end) end)
  end

  def read_test do
    "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.." |> String.split("\n", trim: true)
  end
end

# Solution.part_one(Solution.read_file())
# Solution.part_two(Solution.read_test())
Solution.part_two(Solution.read_file())
