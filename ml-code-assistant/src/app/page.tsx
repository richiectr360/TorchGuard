'use client';

import { useState, useRef } from 'react';
import CodeEditor from '@/components/CodeEditor';
import AnalysisResults from '@/components/AnalysisResults';
import { Editor } from '@monaco-editor/react';

interface Finding {
  category: string;
  message: string;
  line: number;
  severity: string;
  fix?: {
    description: string;
    code: string;
    range: {
      start: number;
      end: number;
    };
  };
}

export default function Home() {
  const [code, setCode] = useState('');
  const [findings, setFindings] = useState<Finding[]>([]);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const editorRef = useRef<any>(null);

  const analyzeCode = async () => {
    setIsAnalyzing(true);
    try {
      const response = await fetch('/api/analyze', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        mode: 'cors',
        credentials: 'omit',
        body: JSON.stringify({ code }),
      });
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      setFindings(data.findings || []);
    } catch (error) {
      console.error('Error analyzing code:', error);
      setFindings([{
        category: 'Error',
        message: 'Failed to analyze code. Please try again.',
        line: 0,
        severity: 'error'
      }]);
    } finally {
      setIsAnalyzing(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">ML Code Assistant</h1>
          <p className="mt-2 text-gray-600">Analyze your PyTorch code for optimization opportunities</p>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-xl font-semibold text-gray-900">Code Editor</h2>
              <button
                onClick={analyzeCode}
                disabled={isAnalyzing || !code.trim()}
                className={`px-4 py-2 rounded-lg font-medium ${isAnalyzing || !code.trim()
                  ? 'bg-gray-300 cursor-not-allowed'
                  : 'bg-blue-600 hover:bg-blue-700 text-white'}`}
              >
                {isAnalyzing ? 'Analyzing...' : 'Analyze Code'}
              </button>
            </div>
            <CodeEditor
              onChange={setCode}
              language="python"
              initialValue="# Paste your PyTorch code here"
              onMount={(editor) => {
                editorRef.current = editor;
              }}
            />
          </div>

          <div className="bg-white p-6 rounded-lg shadow-sm border border-gray-200">
            <AnalysisResults 
              findings={findings} 
              onApplyFix={(fix) => {
                if (fix && editorRef.current) {
                  const editor = editorRef.current;
                  const model = editor.getModel();
                  if (model) {
                    const range = model.getFullModelRange();
                    editor.executeEdits('auto-fix', [{
                      range: range,
                      text: fix.code,
                    }]);
                    setCode(fix.code);
                  }
                }
              }} 
            />
          </div>
        </div>
      </div>
    </div>
  );
}
