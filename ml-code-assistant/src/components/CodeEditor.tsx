'use client';

import React, { useState } from 'react';
import Editor, { OnMount } from '@monaco-editor/react';

interface CodeEditorProps {
  initialValue?: string;
  onChange?: (value: string) => void;
  language?: string;
  onMount?: OnMount;
}

export default function CodeEditor({ 
  initialValue = '', 
  onChange,
  language = 'python',
  onMount
}: CodeEditorProps) {
  const [value, setValue] = useState(initialValue);

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setValue(value);
      onChange?.(value);
    }
  };

  return (
    <div className="h-[500px] w-full border border-gray-300 rounded-lg overflow-hidden">
      <Editor
        height="100%"
        language={language}
        theme="vs-dark"
        value={value}
        onChange={handleEditorChange}
        onMount={onMount}
        options={{
          minimap: { enabled: false },
          fontSize: 14,
          lineNumbers: 'on',
          renderLineHighlight: 'all',
          scrollBeyondLastLine: false,
          automaticLayout: true,
        }}
      />
    </div>
  );
}
