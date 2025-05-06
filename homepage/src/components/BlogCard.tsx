type Props = {
  imageUrl: string;
  children: React.ReactNode;
};

export default function BlogCard({ content, children }: Props) {
  return (
    <figure className="w-[250px] overflow-hidden rounded-base border-2 border-border dark:border-darkBorder bg-main font-base shadow-light dark:shadow-dark">
      <div>{content}</div>
      <figcaption className="border-t-2 text-text border-border dark:border-darkBorder p-4">
        {children}
      </figcaption>
    </figure>
  );
}
