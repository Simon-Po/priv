



defmodule Hey do
 def main do 
case File.read("input") do
      {:ok,contents} -> 
        contents 
        |> IO.puts
        _ -> 0
    end 
  end
end
Hey.main()
