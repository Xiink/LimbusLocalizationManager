import { default as MarkdownComponent } from "react-markdown";

function LinkRenderer(props: React.ComponentPropsWithoutRef<'a'>) {
  const { children, ...rest } = props;
  
  return (
    <a {...rest} target="_blank" rel="noopener noreferrer">
      {children}
    </a>
  );
}

function Markdown({ children }: { children: string }) {
  return (
    <MarkdownComponent components={{ a: LinkRenderer }}>
      {children}
    </MarkdownComponent>
  );
}

export default Markdown;
