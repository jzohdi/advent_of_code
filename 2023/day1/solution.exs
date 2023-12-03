defmodule Solution do
  def part_one do
    {:ok, contents} = File.read("./day1/input.txt")
    contents_to_lines = contents |> String.split("\n", trim: true)
    # IO.inspect(contents_to_lines)
    # IO.inspect(contents_to_lines)
    # get_digits = for line <- contents_to_lines, do: Regex.scan(~r/\d/, line) |> IO.inspect()
    # for line <- contents_to_lines, do: IO.inspect(line)
    sum = List.foldl(contents_to_lines, 0, fn(line, acc) ->
      # IO.inspect(x)
      line = Regex.replace(~r/one/,line, "1")
      line = Regex.replace(~r/two/, line, "2")
      line = Regex.replace(~r/three/, line, "3")
      line = Regex.replace(~r/four/, line, "4")
      line = Regex.replace(~r/five/, line, "5")
      line = Regex.replace(~r/six/, line, "6")
      line = Regex.replace(~r/seven/, line, "7")
      line = Regex.replace(~r/eight/, line, "8")
      line = Regex.replace(~r/nine/, line, "9")
      # digits = List.flatten(digits)
      IO.inspect(line)
      digits = Regex.scan(~r/\d/, line, trim: true) # (&(for x <- &1, do: String.to_integer(x))).()
      digits = List.flatten(digits)
      IO.inspect(digits)
      # digits_to_int = Enum.map(digits, fn(x) ->
      #    {int, rest} = Integer.parse((x))
      #    int
      # end)
      # Enum.map(&1, &(Integer.parse(&1)))
      # acc + Enum.at(digits, 0) + List.last(digits)
      # {first, _rest1} = Integer.parse(List.first(digits))
      {num, _rest2} = Integer.parse(List.first(digits) <> List.last(digits))
      # IO.inspect(num)
      # IO.inspect(first)
      # IO.inspect(last)
      acc + num
    end)
    IO.inspect(sum)
  end
end

Solution.part_one()
