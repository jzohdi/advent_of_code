defmodule Solution do

  @max_cubs %{red: 12, green: 13, blue: 14}
  def read_file do
    File.read!("./day2/input.txt")
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      id = Regex.run(~r/\d+/, line) |> List.first()
      max_for_colors = Regex.replace(~r/Game \d+: /, line, "")
      |> String.split(";", trim: true)
      |> Enum.map(fn round ->
        String.split(round, ", ", trim: true)
        |> Enum.map(fn colors ->
          String.split(colors, " ", trim: true)
        end)
      end)
      {String.to_integer(id), max_for_colors}
    end)
  end

  def part_one do
    contents = File.read!("./day2/input.txt")
              |> String.split("\n", trim: true)
              |> Enum.map(fn line ->
                id = Regex.run(~r/\d+/, line) |> List.first()
                max_for_colors = Regex.replace(~r/Game \d+: /, line, "")
                |> String.split(";", trim: true)
                |> Enum.map(fn round ->
                  String.split(round, ", ", trim: true)
                  |> Enum.map(fn colors ->
                    String.split(colors, " ", trim: true)
                  end)
                end)
                |> Enum.reduce(%{}, fn x, acc ->
                  Enum.reduce(x, acc, fn x2, acc2 ->
                    num = List.first(x2) |> String.to_integer()
                    color = List.last(x2)
                    Map.update(acc2, color, num, fn current -> max(current, num) end)
                  end)
                end)
                {String.to_integer(id), max_for_colors}
              end)

    result = Enum.reduce(contents, 0, fn x, acc ->
       id = elem(x, 0)
      round_is_possible = Enum.reduce(elem(x, 1), true, fn {k, v}, res ->
        max_val = Map.get(@max_cubs,String.to_atom(k))
        res and max_val >= v
      end)
      if round_is_possible do
        acc + id
      else
        acc
      end
    end)
    IO.inspect(result)
  end

  def part_two(lines) do
    result = Enum.map(lines, fn {id, rounds} ->
      max_map = max_color_for_game(rounds)
      {id, max_map}
    end) |> Enum.reduce(0, fn {_id, colors}, acc ->
      acc + Enum.reduce(Map.values(colors), 1, fn x, acc2 -> acc2 * x end)
    end)
    IO.inspect(result)
  end

  def max_color_for_game(rounds) do
    Enum.reduce(rounds, %{}, fn x, acc ->
      Enum.reduce(x, acc, fn x2, acc2 ->
        num = List.first(x2) |> String.to_integer()
        color = List.last(x2)
        Map.update(acc2, color, num, fn current -> max(current, num) end)
      end)
    end)
  end
end

Solution.part_two(Solution.read_file())
