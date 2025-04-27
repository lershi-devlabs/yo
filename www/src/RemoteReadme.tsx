import { useEffect, useState } from 'react';
import ReactMarkdown from 'react-markdown';

function CodeBlock({ children }: { children: React.ReactNode }) {
  const [copied, setCopied] = useState(false);
  const codeString = typeof children === 'string' ? children : (Array.isArray(children) ? children.join('') : '');

  const handleCopy = () => {
    navigator.clipboard.writeText(codeString);
    setCopied(true);
    setTimeout(() => setCopied(false), 1200);
  };

  return (
    <div style={{ position: 'relative', background: '#18181a', borderRadius: 8, padding: '1em', margin: '1.5em 0', fontFamily: 'Geist Mono, monospace', color: '#e5e5e5', overflowX: 'auto' }}>
      <button
        onClick={handleCopy}
        style={{
          position: 'absolute',
          top: 10,
          right: 10,
          background: copied ? '#7fd962' : '#232326',
          color: copied ? '#232326' : '#e5e5e5',
          border: 'none',
          borderRadius: 4,
          padding: '0.2em 0.7em',
          fontSize: '0.9em',
          cursor: 'pointer',
          fontFamily: 'Geist Sans, sans-serif',
          transition: 'background 0.2s, color 0.2s',
        }}
      >
        {copied ? 'Copied!' : 'Copy'}
      </button>
      <pre style={{ margin: 0, background: 'transparent', fontFamily: 'inherit', color: 'inherit' }}>
        <code style={{ fontFamily: 'inherit', color: 'inherit' }}>{children}</code>
      </pre>
    </div>
  );
}

export default function RemoteReadme() {
  const [markdown, setMarkdown] = useState('Loading README...');

  useEffect(() => {
    fetch('https://raw.githubusercontent.com/Montekkundan/yo/master/README.md')
      .then(res => res.text())
      .then(setMarkdown)
      .catch(() => setMarkdown('Failed to load README.'));
  }, []);

  return (
    <div style={{ maxWidth: 900, margin: '0 auto', color: '#222', padding: 32, fontFamily: 'Geist Sans, sans-serif' }}>
      <ReactMarkdown
        components={{
          code(props) {
            return <code style={{ fontFamily: 'Geist Mono, monospace' }}>{props.children}</code>;
          },
          pre({ children }) {
            return <CodeBlock>{children}</CodeBlock>;
          },
        }}
      >
        {markdown}
      </ReactMarkdown>
    </div>
  );
}
