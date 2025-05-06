"use client";

import { ClassValue } from "clsx";

import { cn } from "@/lib/utils";

type Props = {
  className?: ClassValue;
  children: React.ReactNode;
  onClick: (event: React.MouseEvent<HTMLButtonElement>) => void;
};

export default function NavButton({ className, children, onClick }: Props) {
  return (
    <button
      role="button"
      aria-label="Click to perform an action"
      onClick={onClick}
      className={cn(
        "flex text-text  items-center rounded-base border-2 border-border dark:border-darkBorder bg-main px-4 py-1 text-sm font-base shadow-light dark:shadow-dark transition-all hover:translate-x-boxShadowX hover:translate-y-boxShadowY hover:shadow-none dark:hover:shadow-none",
        className
      )}
    >
      {children}
    </button>
  );
}
