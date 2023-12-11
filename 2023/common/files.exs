defmodule Files do
  def to_lines(filename) do
    File.read!("./day2/input.txt")
    |> String.split("\n", trim: true)
  end
end
