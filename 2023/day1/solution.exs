defmodule Solution do
  def part_one do
    num_map  = %{"one" => 1, "two" => 2, "three" =>  3, "four" =>  4, "five" =>  5, "six" =>  6, "seven" =>  7, "eight" => 8, "nine" => 9,
     "1" =>  1, "2" =>  2, "3" =>  3, "4" =>  4, "5" =>  5, "6" =>  6, "7" =>  7, "8" =>  8 , "9" =>  9, "zero"=> 0, "0" =>  0}
    {:ok, contents} = File.read("./day1/input.txt")
    contents_to_lines = contents |> String.split("\n", trim: true)
    sum = List.foldl(contents_to_lines, 0, fn(line, acc) ->
      # IO.inspect(x)
      IO.inspect("parsing line: " <> line)
      digits = Enum.map(0..String.length(line) - 1, fn(index) ->
        matched_num = Enum.find(Map.keys(num_map), fn (k) ->
          key_length = String.length(k)
          sub_string =  String.slice(line, index, key_length)
          sub_string == k
        end)
        val = num_map[matched_num]
        val
      end) |> Enum.filter(fn(ele) -> ele != nil end)
      IO.inspect("result:")
      IO.inspect(digits)
      {num, _rest2} = Integer.parse(Integer.to_string(List.first(digits)) <> Integer.to_string(List.last(digits)))
      acc + num
    end)
    IO.inspect(sum)
  end
end

Solution.part_one()
