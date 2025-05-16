'use client';

import React from 'react';

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

interface AnalysisResultsProps {
  findings: Finding[];
  onApplyFix?: (fix: Finding['fix']) => void;
}

export default function AnalysisResults({ findings, onApplyFix }: AnalysisResultsProps) {
  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'warning':
        return 'bg-yellow-100 text-yellow-800';
      case 'error':
        return 'bg-red-100 text-red-800';
      case 'info':
        return 'bg-blue-100 text-blue-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getCategoryIcon = (category: string) => {
    switch (category.toLowerCase()) {
      case 'gpu usage':
        return '🎮';
      case 'memory usage':
        return '💾';
      case 'performance':
        return '⚡';
      case 'training':
        return '📈';
      case 'model state':
        return '🤖';
      case 'training stability':
        return '🎯';
      default:
        return '💡';
    }
  };

  return (
    <div className="space-y-4">
      <h2 className="text-xl font-semibold mb-4">Analysis Results</h2>
      {findings.length === 0 ? (
        <div className="text-gray-500">No issues found. Your code looks good! 🎉</div>
      ) : (
        <div className="space-y-2">
          {findings.map((finding, index) => (
            <div
              key={index}
              className="p-4 rounded-lg border border-gray-200 hover:border-gray-300 transition-colors"
            >
              <div className="flex items-start gap-3">
                <div className="text-2xl" role="img" aria-label={finding.category}>
                  {getCategoryIcon(finding.category)}
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="font-medium">{finding.category}</span>
                    <span className={`text-xs px-2 py-1 rounded ${getSeverityColor(finding.severity)}`}>
                      {finding.severity}
                    </span>
                    <span className="text-sm text-gray-500">Line {finding.line}</span>
                  </div>
                  <p className="text-gray-700">{finding.message}</p>
                  {finding.fix && (
                    <div className="mt-3 flex items-center gap-3">
                      <button
                        onClick={() => onApplyFix?.(finding.fix)}
                        className="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                      >
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                        </svg>
                        Apply Fix
                      </button>
                      <button
                        onClick={() => {
                          if (finding.fix) {
                            const el = document.createElement('textarea');
                            el.value = finding.fix.code;
                            document.body.appendChild(el);
                            el.select();
                            document.execCommand('copy');
                            document.body.removeChild(el);
                          }
                        }}
                        className="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                      >
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
                        </svg>
                        Copy Fix
                      </button>
                    </div>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
