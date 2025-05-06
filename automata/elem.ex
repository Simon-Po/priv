defmodule Rule30 do
  def run(iterations, initial_state) do
    Enum.reduce(1..iterations, [initial_state], fn _, acc ->
      [next_generation(hd(acc)) | acc]
    end)
    |> Enum.reverse()
    |> Enum.each(&print_generation/1)
  end

  defp next_generation(state) do
    state
    |> Enum.with_index()
    |> Enum.map(fn {_, idx} ->
      left = Enum.at(state, idx - 1, 0)
      center = Enum.at(state, idx, 0)
      right = Enum.at(state, idx + 1, 0)

      case {left, center, right} do
        {1, 1, 1} -> 0
        {1, 1, 0} -> 0
        {1, 0, 1} -> 0
        {1, 0, 0} -> 1
        {0, 1, 1} -> 1
        {0, 1, 0} -> 1
        {0, 0, 1} -> 1
        {0, 0, 0} -> 0
      end
    end)
  end

  defp print_generation(generation) do
    generation
    |> Enum.map(fn
      1 -> "*"
      0 -> " "
    end)
    |> Enum.join("")
    |> IO.puts()
  end
end

initial_state = List.duplicate(0, 20) ++ [1] ++ List.duplicate(0, 20)
Rule30.run(20, initial_state)

