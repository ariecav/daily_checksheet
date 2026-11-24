pub const STYLES: &str = r#"
        :root {
            --primary: #2563eb;
            --primary-hover: #1d4ed8;
            --bg: #f8fafc;
            --card-bg: #ffffff;
            --text: #1e293b;
            --border: #e2e8f0;
            --success: #16a34a;
        }
        body {
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: var(--bg);
            color: var(--text);
            margin: 0;
            padding: 0 20px 20px;
        }
        .container {
            width: 100%;
            max-width: none;
            margin: 0 auto;
            padding-top: 260px;
        }
        .header-fixed-wrapper {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            background-color: var(--bg);
            padding: 20px 20px 0;
            z-index: 1000;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
        }
        header {
            text-align: center;
            margin-bottom: 30px;
        }
        h1 { margin-bottom: 5px; color: var(--primary); }
        .card {
            background: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 24px;
            margin-bottom: 20px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.05);
        }
        .form-group {
            margin-bottom: 15px;
        }
        label {
            display: block;
            font-weight: 600;
            margin-bottom: 5px;
        }
        input[type="text"], textarea, select {
            width: 100%;
            padding: 10px;
            border: 1px solid var(--border);
            border-radius: 6px;
            box-sizing: border-box;
            font-size: 14px;
        }
        textarea { height: 150px; font-family: monospace; }
        button {
            background-color: var(--primary);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 6px;
            font-size: 14px;
            cursor: pointer;
            font-weight: 600;
        }
        button:hover { background-color: var(--primary-hover); }
        .section-box {
            border-left: 4px solid var(--primary);
            padding-left: 15px;
            margin: 20px 0;
        }
        .question-box {
            margin-bottom: 20px;
            padding-bottom: 15px;
            border-bottom: 1px solid var(--border);
        }
        .q-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            gap: 8px;
        }
        .q-header button {
            flex-shrink: 0;
            padding: 6px 12px;
            font-size: 12px;
        }
        .question-body-grid {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }
        @media (min-width: 640px) {
            .question-body-grid.has-media {
                display: grid;
                grid-template-columns: 180px 1fr;
                gap: 20px;
                align-items: center;
            }
        }
        .q-image-preview {
            max-width: 200px;
            max-height: 200px;
            display: block;
            margin: 8px 0;
            border-radius: 6px;
            border: 1px solid var(--border);
        }
        .q-edit-panel {
            margin-top: 12px;
            padding: 12px;
            background: #f8fafc;
            border: 1px solid var(--border);
            border-radius: 6px;
        }
        .options-list label {
            font-weight: normal;
            display: flex;
            align-items: center;
            gap: 8px;
            margin: 6px 0;
            cursor: pointer;
        }
        .results-table-wrapper {
            width: 100%;
            overflow-x: auto;
            margin-top: 10px;
        }
        #results-table {
            width: max-content;
            min-width: 100%;
            border-collapse: collapse;
        }
        th, td {
            border: 1px solid var(--border);
            padding: 10px;
            text-align: left;
            font-size: 14px;
        }
        th { background-color: #f1f5f9; }
        .tab-buttons {
            display: flex;
            gap: 6px;
            margin-bottom: 20px;
            flex-wrap: nowrap;
        }
        .test-context {
            display: grid;
            grid-template-columns: 45% 45%;
            justify-content: flex-start;
            gap: 15px;
            margin-bottom: 28px;
        }
        .context-field label {
            display: block;
            margin-bottom: 5px;
        }
        .name-selector-row {
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .name-selector-row select { flex: 1; min-width: 0; }
        .token-button {
            width: 52px;
            height: 52px;
            padding: 0;
            border-radius: 50%;
            flex: 0 0 52px;
            font-size: 11px;
            line-height: 1;
        }
        .token-button[hidden] { display: none; }
        .tab-btn {
            background: #e2e8f0;
            color: var(--text);
            border: none;
            padding: 6px 10px;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 600;
            font-size: 12px;
            white-space: nowrap;
            flex: 1 1 0;
            min-width: 0;
            text-align: center;
        }
        .tab-btn.active {
            background: var(--primary);
            color: white;
        }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        @media (max-width: 640px) {
            .container { padding-top: 305px; }
            .test-context { grid-template-columns: 1fr; gap: 10px; margin-bottom: 16px; }
            header { margin-bottom: 16px; }
        }
        .alert {
            padding: 12px;
            background: #dcfce7;
            color: #166534;
            border-radius: 6px;
            margin-bottom: 15px;
            font-weight: 500;
        }
        .modal-overlay {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(15, 23, 42, 0.45);
            z-index: 1100;
            align-items: center;
            justify-content: center;
        }
        .modal-overlay.active { display: flex; }
        .modal {
            background: white;
            border-radius: 10px;
            padding: 20px;
            width: 90%;
            max-width: 420px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.25);
        }
        .modal h3 { margin-top: 0; }
        .modal .form-group { margin-bottom: 12px; }
        .token-access-info {
            display: grid;
            gap: 8px;
            padding: 12px;
            border-radius: 6px;
            background: #f1f5f9;
            overflow-wrap: anywhere;
        }
        .token-qr-code {
            display: flex;
            justify-content: center;
            margin: 14px 0;
        }
        .token-qr-code svg {
            width: min(220px, 100%);
            height: auto;
            padding: 8px;
            background: white;
            border: 1px solid var(--border);
            border-radius: 8px;
        }
        .token-access-info a { color: var(--primary); font-weight: 600; }
        .token-note { color: #64748b; font-size: 13px; }
        .modal-actions {
            display: flex;
            gap: 10px;
            justify-content: flex-end;
            margin-top: 18px;
        }
        .btn-secondary {
            background: #e2e8f0;
            color: var(--text);
        }
        .result-actions {
            display: flex;
            gap: 8px;
            flex-wrap: nowrap;
            margin-bottom: 15px;
        }
        .result-actions button {
            flex: 1 1 0;
            min-width: 0;
            padding: 10px 8px;
            font-size: 13px;
            white-space: normal;
            text-align: center;
        }
        .btn-danger {
            background: #dc2626;
        }
        .btn-danger:hover {
            background: #b91c1c;
        }
        @media (max-width: 480px) {
            .result-actions {
                gap: 5px;
            }
            .result-actions button {
                padding: 8px 4px;
                font-size: 11px;
            }
        }
    "#;
