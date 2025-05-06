import NavButton from "@/components/NavButton";
import { cn } from "@/lib/utils";

export default function NavBar({ className, height }) {
  return (
    <div
      className={cn(
        "flex flex-col items-end border-b-2 bg-[#B4B4B8] border-slate-700",
        className
      )}
      style={{ height: height + "rem" }}
    >
      <div className="flex h-full items-center space-x-3 pr-2">
        <NavButton>one</NavButton>
        <NavButton>two</NavButton>
        <NavButton>three</NavButton>
      </div>
    </div>
  );
}
