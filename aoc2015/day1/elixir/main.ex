
# Solution day 1 part 1
# File.read!("input.txt")
#   |> String.graphemes()
#   |> Enum.reduce(0,fn 
#     "(", acc -> acc + 1
#     ")", acc -> acc - 1
#     _,acc -> acc end)
#   |> IO.puts()


# File.read!("input.txt")
# |> String.graphemes()
# |> Enum.reduce_while({0, 0}, fn c, {floor, pos} ->
#   new_floor = floor + if c == "(", do: 1, else: -1
#   if new_floor == -1, do: {:halt, pos + 1}, else: {:cont, {new_floor, pos + 1}}
# end)
# |> IO.puts()


defmodule AddSpaces do
  def add_spaces(input, spaces) do
    spaces_set = MapSet.new(spaces)

    input
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.map(fn {char, index} ->
      if MapSet.member?(spaces_set, index) do
        " " <> char
      else
        char
      end
    end)
    |> Enum.join("")
  end
end

# Example usage:
input = "LeetcodeHelpsMeLearn"
spaces = [8, 13, 15]

result = AddSpaces.add_spaces(input, spaces)
IO.puts(result)
