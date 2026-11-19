pub fn get_index_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Daily Checksheet & Evaluation Test</title>
    <style>
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
            padding: 20px;
        }
        .container {
            width: 100%;
            max-width: none;
            margin: 0 auto;
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
        .q-image {
            max-width: 100%;
            max-height: 280px;
            display: block;
            margin: 10px 0;
            border-radius: 6px;
            border: 1px solid var(--border);
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
            z-index: 100;
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
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Daily Checksheet & Evaluation Portal</h1>
            <p>Digital Evaluation Test System with Automated Scoring</p>
        </header>

        <div id="upload-modal" class="modal-overlay">
            <div class="modal">
                <h3>Upload Test File</h3>
                <div class="form-group">
                    <label for="upload-file">Select JSON file:</label>
                    <input type="file" id="upload-file" accept="application/json">
                </div>
                <div class="form-group">
                    <label for="upload-dept">Dept:</label>
                    <select id="upload-dept">
                        <option value="">-- Select Dept --</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="upload-level">Select Level:</label>
                    <select id="upload-level">
                        <option value="">-- Select Level --</option>
                        <option value="1">1</option>
                        <option value="2">2</option>
                        <option value="3">3</option>
                        <option value="4">4</option>
                    </select>
                </div>
                <div class="modal-actions">
                    <button onclick="submitUpload()">Submit</button>
                    <button class="btn-secondary" onclick="hideUploadModal()">Cancel</button>
                </div>
            </div>
        </div>

        <script>
            function showUploadModal() {
                document.getElementById('upload-modal').classList.add('active');
                // Load departments into the upload modal dropdown
                fetch('/api/departments')
                    .then(res => res.json())
                    .then(depts => {
                        const select = document.getElementById('upload-dept');
                        select.innerHTML = '<option value="">-- Select Dept --</option>';
                        depts.forEach(d => {
                            const opt = document.createElement('option');
                            opt.value = d;
                            opt.textContent = d;
                            select.appendChild(opt);
                        });
                    })
                    .catch(() => {});
            }
            function hideUploadModal() {
                document.getElementById('upload-modal').classList.remove('active');
            }

            async function submitUpload() {
                const fileInput = document.getElementById('upload-file');
                const levelSelect = document.getElementById('upload-level');
                const deptSelect = document.getElementById('upload-dept');

                if (!fileInput.files.length) {
                    alert('Please select a JSON file.');
                    return;
                }

                if (!levelSelect.value) {
                    alert('Please select a level.');
                    return;
                }

                if (!deptSelect.value) {
                    alert('Please select a department.');
                    return;
                }

                const file = fileInput.files[0];
                const reader = new FileReader();

                reader.onload = async function(event) {
                    try {
                        const testJson = JSON.parse(event.target.result);
                        testJson.level = parseInt(levelSelect.value);
                        testJson.department = deptSelect.value;

                        const res = await fetch('/api/tests', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(testJson)
                        });

                        if (res.ok) {
                            alert('Test uploaded successfully!');
                            hideUploadModal();
                            fileInput.value = '';
                            levelSelect.value = '';
                            deptSelect.value = '';
                        } else {
                            const error = await res.text();
                            alert('Upload failed: ' + error);
                        }
                    } catch(e) {
                        alert('Error reading or parsing file: ' + e.message);
                    }
                };

                reader.readAsText(file);
            }
        </script>

        <div class="test-context">
            <div class="context-field">
                <label for="section-dept">Section Dept:</label>
                <select id="section-dept" onchange="loadTestList()">
                    <option value="">-- Select Section Dept --</option>
                </select>
            </div>
            <div class="context-field">
                <label for="taker-name">Name:</label>
                <input type="text" id="taker-name" placeholder="Enter your full name">
            </div>
        </div>

        <div class="tab-buttons">
            <button class="tab-btn active" data-tab="take-test" onclick="switchTab('take-test')">Take Test</button>
            <button class="tab-btn" data-tab="upload-json" onclick="switchTab('upload-json')">Upload Test</button>
            <button class="tab-btn" data-tab="bank-test" onclick="switchTab('bank-test')">Bank Test</button>
            <button class="tab-btn" data-tab="results-view" onclick="switchTab('results-view')">Result</button>
        </div>

        <!-- TAKE TEST TAB -->
        <div id="take-test" class="tab-content active">
            <div id="edit-mode-banner" class="card" style="display:none; background:#fff7ed; border-color:#fdba74;">
                <div style="display:flex; justify-content:space-between; align-items:center; gap:12px; flex-wrap:wrap;">
                    <strong>Editing Test: <span id="edit-mode-test-title"></span></strong>
                    <button type="button" class="btn-secondary" onclick="exitEditMode()">Back to Bank Test</button>
                </div>
            </div>
            <div class="card">
                <script>
                    function refreshModeOptions() {
                        const modeSelect = document.getElementById('select-mode');
                        const selectedMode = modeSelect.value;
                        console.log('Selected mode:', selectedMode);
                        // Placeholder: Implement dynamic refresh logic here
                    }
                </script>
                <h2 id="take-test-heading">Select & Take Evaluation Test</h2>
                <div id="take-test-mode-level-group" class="form-group" style="display: flex; gap: 12px; align-items: center;">
                    <div style="flex:1">
                        <label for="select-mode">Select Mode:</label>
                        <select id="select-mode" onchange="refreshModeOptions()">
                            <option value="">-- Select Mode --</option>
                            <option value="refresh">Refresh</option>
                            <option value="upgrade">Upgrade</option>
                        </select>
                    </div>
                    <div style="flex:1">
                        <label for="select-level">Select Level:</label>
                        <select id="select-level" onchange="loadTestList()">
                            <option value="">-- Select Level --</option>
                            <option value="1">1</option>
                            <option value="2">2</option>
                            <option value="3">3</option>
                            <option value="4">4</option>
                        </select>
                    </div>
                </div>
                <div id="take-test-select-group" class="form-group">
                    <label for="select-test-id">Available Tests:</label>
                    <select id="select-test-id" onchange="loadTestForTaking()">
                        <option value="">-- Select a Test --</option>
                    </select>
                </div>
                <div id="test-form-container"></div>
                <button id="submit-test-btn" style="display:none;" onclick="submitTest()">Submit Test</button>
            </div>
            <div id="result-display" class="card" style="display:none;"></div>
        </div>

        <!-- UPLOAD TEST TAB -->
        <div id="upload-json" class="tab-content">
            <div class="card">
                <h2>Upload Test JSON Structure</h2>
                <button onclick="showUploadModal()">Upload File</button>
                <div id="upload-alert" class="alert" style="display:none;">Test uploaded successfully!</div>
                <div class="form-group">
                    <textarea id="json-input">{
  "test_id": "eval_001",
  "test_title": "Daily Safety & Operations Evaluation",
  "sections": [
    {
      "section_id": "sec_1",
      "section_name": "General Safety Compliance",
      "questions": [
        {
          "id": "q1",
          "question_text": "What is the primary action when machinery sparks?",
          "options": ["A) Ignore it", "B) Stop and report immediately", "C) Pour water", "D) Continue working"],
          "correct_answer": "B",
          "points": 10
        },
        {
          "id": "q2",
          "question_text": "Which PPE is mandatory in workshop area?",
          "options": ["A) Safety Helmet & Glasses", "B) Sandals", "C) Sunglasses", "D) Casual Hat"],
          "correct_answer": "A",
          "points": 10
        }
      ]
    }
  ]
}</textarea>
                </div>
            </div>
        </div>

        <!-- BANK TEST TAB -->
        <div id="bank-test" class="tab-content">
            <div class="card">
                <h2>Bank Test (Uploaded Questions)</h2>
                <div class="result-actions">
                    <button onclick="loadBankTests()">Refresh List</button>
                    <button onclick="openDeleteBankModal()" class="btn-danger">Delete Selected</button>
                    <button onclick="deleteAllBankTests()" class="btn-danger">Delete All</button>
                </div>
                <div class="results-table-wrapper">
                    <table id="bank-test-table">
                        <thead>
                            <tr>
                                <th><input type="checkbox" id="select-all-bank" onchange="toggleAllBank(this)"></th>
                                <th>Test ID</th>
                                <th>Title</th>
                                <th>Level</th>
                                <th>Dept</th>
                                <th>Status</th>
                            </tr>
                        </thead>
                        <tbody></tbody>
                    </table>
                </div>
            </div>
        </div>

        <div id="delete-bank-modal" class="modal-overlay">
            <div class="modal">
                <h3>Delete Selected Tests</h3>
                <p>Are you sure you want to delete selected tests from the bank?</p>
                <div class="modal-actions">
                    <button class="btn-secondary" onclick="closeDeleteBankModal()">Cancel</button>
                    <button class="btn-danger" onclick="deleteSelectedBankTests()">Delete</button>
                </div>
            </div>
        </div>
        <!-- RESULTS TAB -->
        <div id="results-view" class="tab-content">
            <div class="card">
                <h2>Test Submission Results (SQLite3 Database)</h2>
                <div class="result-actions">
                    <button onclick="loadResults()">Refresh Results</button>
                    <button onclick="openDeleteModal()" class="btn-danger">Delete Selected</button>
                    <button onclick="deleteAllResults()" class="btn-danger">Delete All</button>
                </div>
                <div class="results-table-wrapper">
                    <table id="results-table">
                        <thead>
                            <tr>
                                <th><input type="checkbox" id="select-all-results" onchange="toggleAllResults(this)"></th>
                                <th>ID</th>
                                <th>Test ID</th>
                                <th>User Name</th>
                                <th>Score</th>
                                <th>Max Score</th>
                                <th>Percentage</th>
                                <th>Mode</th>
                                <th>Level</th>
                                <th>Test Date</th>
                                <th>Fiscal Year</th>
                                <th>Evaluated At</th>
                            </tr>
                        </thead>
                        <tbody>
                            <!-- Loaded dynamically -->
                        </tbody>
                    </table>
                </div>
            </div>
        </div>

        <div id="delete-results-modal" class="modal-overlay">
            <div class="modal">
                <h3>Delete Selected Results</h3>
                <p>Are you sure you want to delete the selected results?</p>
                <div class="modal-actions">
                    <button class="btn-secondary" onclick="closeDeleteModal()">Cancel</button>
                    <button class="btn-danger" onclick="deleteSelectedResults()">Delete</button>
                </div>
            </div>
        </div>

    </div>

    <script>
        function switchTab(tabId, skipTakeTestReset) {
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            const btn = document.querySelector(`.tab-btn[data-tab="${tabId}"]`);
            if (btn) btn.classList.add('active');
            document.getElementById(tabId).classList.add('active');
            if (tabId === 'take-test') {
                if (!skipTakeTestReset) {
                    setEditModeUI(false);
                    loadTestList();
                }
            } else if (tabId === 'results-view') {
                loadResults();
            }
        }

        let editMode = false;

        function setEditModeUI(active, title) {
            editMode = active;
            document.getElementById('edit-mode-banner').style.display = active ? 'block' : 'none';
            document.getElementById('take-test-heading').style.display = active ? 'none' : 'block';
            document.getElementById('take-test-mode-level-group').style.display = active ? 'none' : 'flex';
            document.getElementById('take-test-select-group').style.display = active ? 'none' : 'block';
            if (active) {
                document.getElementById('edit-mode-test-title').textContent = title || '';
            }
        }

        async function editBankTest(testId) {
            switchTab('take-test', true);
            setEditModeUI(true, testId);
            await loadTestForTaking(testId);
        }

        function exitEditMode() {
            setEditModeUI(false);
            document.getElementById('test-form-container').innerHTML = '';
            document.getElementById('submit-test-btn').style.display = 'none';
            switchTab('bank-test');
        }

        async function uploadTestJson() {
            const jsonText = document.getElementById('json-input').value;
            try {
                const jsonObj = JSON.parse(jsonText);
                const res = await fetch('/api/tests', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(jsonObj)
                });
                const data = await res.json();
                if (res.ok) {
                    const alertBox = document.getElementById('upload-alert');
                    alertBox.style.display = 'block';
                    setTimeout(() => alertBox.style.display = 'none', 3000);
                    loadTestList();
                } else {
                    alert('Error: ' + data.message);
                }
            } catch (err) {
                alert('Invalid JSON: ' + err.message);
            }
        }

        async function loadDepartmentList() {
            try {
                const res = await fetch('/api/departments');
                if (!res.ok) throw new Error('Failed to load departments');
                const departments = await res.json();
                const select = document.getElementById('section-dept');
                select.innerHTML = '<option value="">-- Select Section Dept --</option>';
                departments.forEach(department => {
                    const option = document.createElement('option');
                    option.value = department;
                    option.textContent = department;
                    select.appendChild(option);
                });
            } catch (err) {
                console.error('Failed to load department list', err);
            }
        }

        async function loadTestList() {
            try {
                const res = await fetch('/api/tests');
                const tests = await res.json();
                const select = document.getElementById('select-test-id');
                const dept = document.getElementById('section-dept')?.value || '';
                const level = document.getElementById('select-level')?.value || '';

                const filtered = tests.filter(t => {
                    if (dept && (t.department || '') !== dept) return false;
                    if (level && (t.level || 0) !== Number(level)) return false;
                    return true;
                });

                select.innerHTML = '<option value="">-- Select a Test --</option>';
                filtered.forEach(t => {
                    const opt = document.createElement('option');
                    opt.value = t.test_id;
                    opt.textContent = t.test_id + ' - ' + t.test_title;
                    select.appendChild(opt);
                });
            } catch (err) {
                console.error('Failed to load test list', err);
            }
        }

        let currentTestObj = null;

        async function loadTestForTaking(testIdOverride) {
            const testId = testIdOverride || document.getElementById('select-test-id').value;
            const container = document.getElementById('test-form-container');
            const submitBtn = document.getElementById('submit-test-btn');
            document.getElementById('result-display').style.display = 'none';

            if (!testId) {
                container.innerHTML = '';
                submitBtn.style.display = 'none';
                return;
            }

            const res = await fetch(`/api/tests/${testId}`);
            if (!res.ok) return;
            currentTestObj = await res.json();

            if (editMode) {
                document.getElementById('edit-mode-test-title').textContent =
                    `${currentTestObj.test_id} - ${currentTestObj.test_title}`;
            }

            let html = `<h3>${currentTestObj.test_title}</h3>`;
            currentTestObj.sections.forEach(sec => {
                html += `<div class="section-box"><h4>Section: ${sec.section_name}</h4>`;
                sec.questions.forEach((q, idx) => {
                    html += `<div class="question-box" data-q-id="${q.id}">
                        <div class="q-header">
                            <p><strong>Q${idx + 1}: ${q.question_text}</strong> (${q.points} pts)</p>
                            ${editMode ? `<button type="button" class="btn-secondary" onclick="toggleEditQuestion('${q.id}')">Edit</button>` : ''}
                        </div>
                        ${q.image ? `<img src="${q.image}" class="q-image">` : ''}
                        <div class="options-list">`;
                    q.options.forEach(opt => {
                        html += `<label><input type="radio" name="q_${q.id}" value="${opt}"> ${opt}</label>`;
                    });
                    html += `</div>`;
                    if (editMode) {
                        html += `<div class="q-edit-panel" id="edit-panel-${q.id}" style="display:none;">
                            <div class="form-group">
                                <label>Edit Question Text:</label>
                                <textarea id="edit-text-${q.id}">${escapeHtml(q.question_text)}</textarea>
                            </div>
                            <div class="form-group">
                                <label>Image (optional):</label>
                                <input type="file" accept="image/*" onchange="onQuestionImageSelected('${q.id}', this)">
                                <input type="hidden" id="edit-img-data-${q.id}" value="${q.image || ''}">
                                <img id="preview-${q.id}" class="q-image-preview" src="${q.image || ''}" style="${q.image ? '' : 'display:none;'}">
                                ${q.image ? `<button type="button" class="btn-secondary" onclick="clearQuestionImage('${q.id}')">Remove Image</button>` : ''}
                            </div>
                            <button type="button" onclick="saveQuestionEdit('${currentTestObj.test_id}', '${q.id}')">Save</button>
                            <button type="button" class="btn-secondary" onclick="toggleEditQuestion('${q.id}')">Cancel</button>
                        </div>`;
                    }
                    html += `</div>`;
                });
                html += `</div>`;
            });

            container.innerHTML = html;
            submitBtn.style.display = editMode ? 'none' : 'block';
        }

        function escapeHtml(str) {
            return String(str)
                .replace(/&/g, '&amp;')
                .replace(/</g, '&lt;')
                .replace(/>/g, '&gt;')
                .replace(/"/g, '&quot;');
        }

        function toggleEditQuestion(qId) {
            const panel = document.getElementById(`edit-panel-${qId}`);
            if (panel) panel.style.display = panel.style.display === 'none' ? 'block' : 'none';
        }

        function resizeImageToDataUri(file, maxDim = 800, quality = 0.8) {
            return new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = (e) => {
                    const img = new Image();
                    img.onload = () => {
                        let { width, height } = img;
                        if (width > maxDim || height > maxDim) {
                            if (width > height) {
                                height = Math.round(height * maxDim / width);
                                width = maxDim;
                            } else {
                                width = Math.round(width * maxDim / height);
                                height = maxDim;
                            }
                        }
                        const canvas = document.createElement('canvas');
                        canvas.width = width;
                        canvas.height = height;
                        canvas.getContext('2d').drawImage(img, 0, 0, width, height);
                        resolve(canvas.toDataURL('image/jpeg', quality));
                    };
                    img.onerror = reject;
                    img.src = e.target.result;
                };
                reader.onerror = reject;
                reader.readAsDataURL(file);
            });
        }

        async function onQuestionImageSelected(qId, inputEl) {
            const file = inputEl.files[0];
            if (!file) return;
            try {
                const dataUri = await resizeImageToDataUri(file);
                document.getElementById(`edit-img-data-${qId}`).value = dataUri;
                const preview = document.getElementById(`preview-${qId}`);
                preview.src = dataUri;
                preview.style.display = 'block';
            } catch (err) {
                alert('Failed to process image: ' + err.message);
            }
        }

        function clearQuestionImage(qId) {
            document.getElementById(`edit-img-data-${qId}`).value = '';
            const preview = document.getElementById(`preview-${qId}`);
            preview.style.display = 'none';
            preview.src = '';
        }

        async function saveQuestionEdit(testId, qId) {
            const text = document.getElementById(`edit-text-${qId}`).value.trim();
            if (!text) {
                alert('Question text cannot be empty');
                return;
            }
            const imageVal = document.getElementById(`edit-img-data-${qId}`).value;
            try {
                const res = await fetch(`/api/tests/${testId}/questions/${qId}`, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ question_text: text, image: imageVal || null })
                });
                if (!res.ok) {
                    const err = await res.json().catch(() => ({}));
                    alert('Failed to save: ' + (err.error || res.statusText));
                    return;
                }
                await loadTestForTaking(testId);
            } catch (err) {
                alert('Error saving question: ' + err.message);
            }
        }

        async function submitTest() {
            if (!currentTestObj) return;
            const userName = document.getElementById('taker-name').value.trim();
            if (!userName) {
                alert('Please enter your name');
                return;
            }

            const answers = [];
            currentTestObj.sections.forEach(sec => {
                sec.questions.forEach(q => {
                    const selected = document.querySelector(`input[name="q_${q.id}"]:checked`);
                    answers.push({
                        question_id: q.id,
                        selected_answer: selected ? selected.value : ""
                    });
                });
            });

            const selectMode = document.getElementById('select-mode');
            const selectLevel = document.getElementById('select-level');
            const submission = {
                test_id: currentTestObj.test_id,
                user_name: userName,
                answers: answers,
                mode: selectMode.value || null,
                level: selectLevel.value ? parseInt(selectLevel.value) : null
            };

            const res = await fetch('/api/submit', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(submission)
            });

            const data = await res.json();
            if (res.ok) {
                const resBox = document.getElementById('result-display');
                resBox.style.display = 'block';
                resBox.innerHTML = `
                    <h3>Evaluation Result</h3>
                    <p><strong>Candidate:</strong> ${data.user_name}</p>
                    <p><strong>Test ID:</strong> ${data.test_id}</p>
                    <p><strong>Total Score:</strong> ${data.total_score} / ${data.max_score}</p>
                    <p><strong>Percentage:</strong> ${data.percentage.toFixed(2)}%</p>
                    <p><strong>Evaluated At:</strong> ${new Date(data.evaluated_at).toLocaleString()}</p>
                `;
            } else {
                alert('Submission error: ' + data.error);
            }
        }

        function toggleAllResults(source) {
            document.querySelectorAll('.result-checkbox').forEach(checkbox => {
                checkbox.checked = source.checked;
            });
        }

        function openDeleteModal() {
            const selected = document.querySelectorAll('.result-checkbox:checked');
            if (!selected.length) {
                alert('Please select at least one result.');
                return;
            }
            document.getElementById('delete-results-modal').classList.add('active');
        }

        function closeDeleteModal() {
            document.getElementById('delete-results-modal').classList.remove('active');
        }

        async function deleteSelectedResults() {
            const ids = Array.from(document.querySelectorAll('.result-checkbox:checked'))
                .map(checkbox => Number(checkbox.value));
            const res = await fetch('/api/results', {
                method: 'DELETE',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(ids)
            });
            if (!res.ok) {
                alert('Failed to delete selected results.');
                return;
            }
            closeDeleteModal();
            await loadResults();
        }

        async function deleteAllResults() {
            if (!confirm('Are you sure you want to delete all results?')) return;
            const res = await fetch('/api/results/all', { method: 'DELETE' });
            if (!res.ok) {
                alert('Failed to delete all results.');
                return;
            }
            await loadResults();
        }

        async function loadBankTests() {
            try {
                const res = await fetch('/api/tests');
                const tests = await res.json();
                const tbody = document.querySelector('#bank-test-table tbody');
                tbody.innerHTML = '';
                tests.forEach(t => {
                    const tr = document.createElement('tr');
                    tr.innerHTML = `
                        <td><input type="checkbox" class="bank-checkbox" value="${t.test_id}"></td>
                        <td>${t.test_id}</td>
                        <td>${t.test_title}</td>
                        <td>${t.level || '-'}</td>
                        <td>${t.department || '-'}</td>
                        <td><button type="button" class="btn-secondary" onclick="editBankTest('${t.test_id}')">Edit</button></td>
                    `;
                    tbody.appendChild(tr);
                });
            } catch (err) { console.error('Error loading bank tests:', err); }
        }

        function toggleAllBank(source) {
            document.querySelectorAll('.bank-checkbox').forEach(c => c.checked = source.checked);
        }

        function openDeleteBankModal() {
            const selected = document.querySelectorAll('.bank-checkbox:checked');
            if (!selected.length) return alert('Please select at least one test');
            document.getElementById('delete-bank-modal').classList.add('active');
        }

        function closeDeleteBankModal() {
            document.getElementById('delete-bank-modal').classList.remove('active');
        }

        async function deleteSelectedBankTests() {
            const selected = Array.from(document.querySelectorAll('.bank-checkbox:checked'))
                .map(c => c.value);
            const res = await fetch('/api/tests', {
                method: 'DELETE',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(selected)
            });
            if (!res.ok) {
                alert('Failed to delete selected tests');
                return;
            }
            closeDeleteBankModal();
            await loadBankTests();
        }

        async function deleteAllBankTests() {
            if (!confirm('Are you sure you want to delete all tests?')) return;
            const res = await fetch('/api/tests/all', { method: 'DELETE' });
            if (!res.ok) {
                alert('Failed to delete all tests');
                return;
            }
            await loadBankTests();
        }

        async function loadResults() {
            try {
                const res = await fetch('/api/results');
                const results = await res.json();
                const tbody = document.querySelector('#results-table tbody');
                tbody.innerHTML = '';
                results.forEach(r => {
                    const tr = document.createElement('tr');
                    tr.innerHTML = `
                        <td><input type="checkbox" class="result-checkbox" value="${r.submission_id}"></td>
                        <td>${r.submission_id}</td>
                        <td>${r.test_id}</td>
                        <td>${r.user_name}</td>
                        <td>${r.total_score}</td>
                        <td>${r.max_score}</td>
                        <td>${r.percentage.toFixed(2)}%</td>
                        <td>${r.mode || '-'}</td>
                        <td>${r.level || '-'}</td>
                        <td>${r.test_date || '-'}</td>
                        <td>${r.fiscal_year || '-'}</td>
                        <td>${new Date(r.evaluated_at).toLocaleString()}</td>
                    `;
                    tbody.appendChild(tr);
                });
            } catch (err) {
                console.error('Failed to load results', err);
            }
        }

        window.onload = () => {
            loadDepartmentList();
            loadTestList();
        };
    </script>
</body>
</html>
"#
}
