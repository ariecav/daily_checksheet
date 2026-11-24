pub const SCRIPTS: &str = r#"
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
            const testNameInput = document.getElementById('upload-test-name');
            const levelSelect = document.getElementById('upload-level');
            const deptSelect = document.getElementById('upload-dept');

            if (!fileInput.files.length) {
                alert('Please select a JSON file.');
                return;
            }

            if (!testNameInput.value.trim()) {
                alert('Please enter the test name.');
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
                    testJson.test_name = testNameInput.value.trim();
                    testJson.level = levelSelect.value ? parseInt(levelSelect.value) : null;
                    testJson.department = deptSelect.value;

                    const res = await fetch('/api/tests', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify(testJson)
                    });

                    if (res.ok) {
                        const result = await res.json();
                        alert(`Test uploaded successfully! Bank Test ID: ${result.test_id}`);
                        hideUploadModal();
                        fileInput.value = '';
                        testNameInput.value = '';
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

        function refreshModeOptions() {
            const modeSelect = document.getElementById('select-mode');
            const selectedMode = modeSelect.value;
            console.log('Selected mode:', selectedMode);
            // Placeholder: Implement dynamic refresh logic here
        }

        function switchTab(tabId, skipTakeTestReset) {
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            const btn = document.querySelector(`.tab-btn[data-tab="${tabId}"]`);
            if (btn) btn.classList.add('active');
            document.getElementById(tabId).classList.add('active');
            if (tabId === 'take-test') {
                if (!skipTakeTestReset) {
                    setEditModeUI(false);
                    onEvaluationFormChange();
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
            document.getElementById('evaluation-form-group').style.display = active ? 'none' : 'block';
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
                const res = await fetch('/api/participant-sections');
                if (!res.ok) throw new Error('Failed to load section depts');
                const sections = await res.json();
                const select = document.getElementById('section-dept');
                select.innerHTML = '<option value="">-- Select Section Dept --</option>';
                sections.forEach(section => {
                    const option = document.createElement('option');
                    option.value = section;
                    option.textContent = section;
                    select.appendChild(option);
                });
            } catch (err) {
                console.error('Failed to load section dept list', err);
            }
        }

        let participantRoles = {};

        function applyTabAccess(role) {
            const isAdmin = role === 'admin';
            const tokenButton = document.getElementById('token-button');
            if (tokenButton) tokenButton.hidden = !isAdmin;
            document.querySelectorAll('.tab-btn').forEach(btn => {
                if (btn.dataset.tab === 'take-test') return;
                btn.style.display = isAdmin ? '' : 'none';
            });
            if (!isAdmin) {
                const activeBtn = document.querySelector('.tab-btn.active');
                if (activeBtn && activeBtn.dataset.tab !== 'take-test') {
                    switchTab('take-test');
                }
            }
        }

        function onTakerNameChange() {
            const name = document.getElementById('taker-name').value;
            applyTabAccess(participantRoles[name] || '');
        }

        async function showTokenModal() {
            const modal = document.getElementById('token-modal');
            const info = document.getElementById('token-access-info');
            const qrCode = document.getElementById('token-qr-code');
            modal.classList.add('active');
            info.textContent = 'Memuat alamat IP...';
            qrCode.replaceChildren();

            try {
                const response = await fetch('/api/access-info');
                if (!response.ok) throw new Error('Gagal mengambil alamat IP');
                const data = await response.json();
                if (!data.access_points?.length) {
                    info.textContent = `Alamat IPv4 tidak ditemukan. Gunakan IP komputer server dengan port ${data.port}.`;
                    return;
                }
                const [primaryAccessPoint] = data.access_points;
                if (primaryAccessPoint.qr_svg) {
                    qrCode.innerHTML = primaryAccessPoint.qr_svg;
                }
                info.replaceChildren(...data.access_points.map(({ url }) => {
                    const link = document.createElement('a');
                    link.href = url;
                    link.textContent = url;
                    link.target = '_blank';
                    link.rel = 'noopener';
                    return link;
                }));
            } catch (error) {
                info.textContent = 'Alamat IP tidak dapat dimuat. Periksa apakah perintah ifconfig tersedia di server.';
            }
        }

        function hideTokenModal() {
            document.getElementById('token-modal')?.classList.remove('active');
        }

        function closeTokenModalOnBackdrop(event) {
            if (event.target.id === 'token-modal') hideTokenModal();
        }

        async function loadParticipantNames() {
            const section = document.getElementById('section-dept')?.value || '';
            const select = document.getElementById('taker-name');
            select.innerHTML = '<option value="">-- Select Name --</option>';
            participantRoles = {};
            applyTabAccess('');
            if (!section) return;
            try {
                const res = await fetch(`/api/participants?section=${encodeURIComponent(section)}`);
                if (!res.ok) throw new Error('Failed to load participants');
                const participants = await res.json();
                participants.forEach(p => {
                    participantRoles[p.name] = p.role;
                    const option = document.createElement('option');
                    option.value = p.name;
                    option.textContent = p.name;
                    select.appendChild(option);
                });
            } catch (err) {
                console.error('Failed to load participant names', err);
            }
        }

        function resetTakeTestControls(resetEvalForm = false) {
            if (resetEvalForm) {
                const evalForm = document.getElementById('select-evaluation-form');
                if (evalForm) {
                    evalForm.value = '';
                }
            }
            ['select-mode', 'select-level', 'select-test-id'].forEach(id => {
                const el = document.getElementById(id);
                if (el) {
                    el.disabled = true;
                    el.value = '';
                }
            });
            const testIdSelect = document.getElementById('select-test-id');
            if (testIdSelect) {
                testIdSelect.innerHTML = '<option value="">-- Select a Test --</option>';
            }
            document.getElementById('test-form-container').innerHTML = '';
            document.getElementById('submit-test-btn').style.display = 'none';
            document.getElementById('result-display').style.display = 'none';
        }

        function onEvaluationFormChange() {
            resetTakeTestControls();
            const hasForm = !!document.getElementById('select-evaluation-form').value;
            if (!hasForm) return;
            ['select-mode', 'select-level', 'select-test-id'].forEach(id => {
                document.getElementById(id).disabled = false;
            });
            loadTestList();
        }

        async function loadEvaluationFormOptions() {
            try {
                const res = await fetch('/api/departments');
                if (!res.ok) throw new Error('Failed to load evaluation forms');
                const departments = await res.json();
                const select = document.getElementById('select-evaluation-form');
                select.innerHTML = '<option value="">-- Select Evaluation Form --</option>';
                departments.forEach(department => {
                    const option = document.createElement('option');
                    option.value = department;
                    option.textContent = department;
                    select.appendChild(option);
                });
            } catch (err) {
                console.error('Failed to load evaluation form list', err);
            }
        }

        async function loadTestList() {
            try {
                const res = await fetch('/api/tests');
                const tests = await res.json();
                const select = document.getElementById('select-test-id');
                const dept = document.getElementById('select-evaluation-form')?.value || '';
                const level = document.getElementById('select-level')?.value || '';

                const filtered = tests.filter(t => {
                    if (dept && (t.department || '') !== dept) return false;
                    if (level && (t.level || 0) !== 0 && (t.level || 0) !== Number(level)) return false;
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
                    const hasMedia = !!q.image;
                    html += `<div class="question-box" data-q-id="${q.id}">
                        <div class="q-header">
                            <p><strong>Q${idx + 1}: ${q.question_text}</strong> (${q.points} pts)</p>
                            ${editMode ? `<button type="button" class="btn-secondary" onclick="toggleEditQuestion('${q.id}')">Edit</button>` : ''}
                        </div>
                        <div class="question-body-grid ${hasMedia ? 'has-media' : ''}">
                            ${hasMedia ? `<div class="question-media-pane">` + (
                                q.image.startsWith('data:video/') ? `<video src="${q.image}" class="q-image" controls autoplay loop style="width:150px; max-height:150px; object-fit:cover;"></video>` :
                                `<img src="${q.image}" class="q-image" style="width:150px; max-height:150px; object-fit:cover;">`
                            ) + `</div>` : ''}
                            <div class="question-options-pane">
                                <div class="options-list">`;
                    q.options.forEach(opt => {
                        html += `<label><input type="radio" name="q_${q.id}" value="${opt}"> ${opt}</label>`;
                    });
                    html += `</div>
                            </div>
                        </div>`;
                    if (editMode) {
                        html += `<div class="q-edit-panel" id="edit-panel-${q.id}" style="display:none;">
                            <div class="form-group">
                                <label>Edit Question Text:</label>
                                <textarea id="edit-text-${q.id}">${escapeHtml(q.question_text)}</textarea>
                            </div>
                            <div class="form-group">
                                <label>Media / Image / Video (optional):</label>
                                <input type="file" accept="image/*,video/*,application/pdf" onchange="onQuestionImageSelected('${q.id}', this)">
                                <input type="hidden" id="edit-img-data-${q.id}" value="${q.image || ''}">
                                <div id="media-preview-container-${q.id}" style="${q.image ? '' : 'display:none;'} margin-top:8px;">
                                    ${q.image ? (
                                        q.image.startsWith('data:video/') ? `<video src="${q.image}" class="q-image" controls style="max-height:280px; width:100%;"></video><button type="button" class="btn-secondary" onclick="clearQuestionImage('${q.id}')">Remove Video</button>` :
                                        `<img id="preview-${q.id}" class="q-image-preview" src="${q.image}" style="display:block;"><button type="button" class="btn-secondary" onclick="clearQuestionImage('${q.id}')">Remove Media</button>`
                                    ) : ''}
                                </div>
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
            
            // Check file size (e.g., max 25MB for video/large media)
            const maxSize = 25 * 1024 * 1024;
            if (file.size > maxSize) {
                alert('File size exceeds 25MB limit. Please choose a smaller video or compress it.');
                inputEl.value = '';
                return;
            }

            const reader = new FileReader();
            reader.onload = async function(e) {
                const dataUri = e.target.result;
                // If it's an image, resize it via canvas for smaller size
                if (file.type.startsWith('image/')) {
                    try {
                        const resized = await resizeImageToDataUri(file);
                        document.getElementById(`edit-img-data-${qId}`).value = resized;
                        updateMediaPreview(qId, resized, 'image');
                        return;
                    } catch (err) {
                        console.warn('Image resize failed, using original dataUri', err);
                    }
                }
                // For video, pdf, or other files, use base64 directly
                document.getElementById(`edit-img-data-${qId}`).value = dataUri;
                const mediaType = file.type.startsWith('video/') ? 'video' : (file.type === 'application/pdf' ? 'pdf' : 'file');
                updateMediaPreview(qId, dataUri, mediaType);
            };
            reader.onerror = (err) => alert('Failed to read file: ' + err.message);
            reader.readAsDataURL(file);
        }

        function updateMediaPreview(qId, dataUri, type) {
            const container = document.getElementById(`media-preview-container-${qId}`);
            if (!container) return;
            if (type === 'video') {
                container.innerHTML = `<video src="${dataUri}" class="q-image" controls style="max-height:280px; width:100%;"></video>
                    <button type="button" class="btn-secondary" onclick="clearQuestionImage('${qId}')">Remove Video</button>`;
            } else if (type === 'pdf') {
                container.innerHTML = `<p style="font-size:12px; color:#475569;">PDF Document Attached</p>
                    <button type="button" class="btn-secondary" onclick="clearQuestionImage('${qId}')">Remove PDF</button>`;
            } else {
                container.innerHTML = `<img id="preview-${qId}" class="q-image-preview" src="${dataUri}" style="display:block;">
                    <button type="button" class="btn-secondary" onclick="clearQuestionImage('${qId}')">Remove Media</button>`;
            }
            container.style.display = 'block';
        }

        function clearQuestionImage(qId) {
            document.getElementById(`edit-img-data-${qId}`).value = '';
            const container = document.getElementById(`media-preview-container-${qId}`);
            if (container) {
                container.innerHTML = '';
                container.style.display = 'none';
            }
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
                        <td>${t.test_name || t.test_title}</td>
                        <td>${t.level || 'All'}</td>
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
                window._currentResults = results;
                const tbody = document.querySelector('#results-table tbody');
                tbody.innerHTML = '';
                results.forEach(r => {
                    const tr = document.createElement('tr');
                    tr.innerHTML = `
                        <td><input type="checkbox" class="result-checkbox" value="${r.submission_id}"></td>
                        <td>${r.submission_id}</td>
                        <td>${r.test_id}</td>
                        <td>${r.test_name || '-'}</td>
                        <td>${r.user_name}</td>
                        <td>${r.section || '-'}</td>
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

        function downloadResultsJson() {
            const data = window._currentResults || [];
            if (!data.length) {
                alert('No results available in the table to download.');
                return;
            }
            const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(data, null, 2));
            const downloadAnchor = document.createElement('a');
            downloadAnchor.setAttribute("href", dataStr);
            downloadAnchor.setAttribute("download", "test_submission_results.json");
            document.body.appendChild(downloadAnchor);
            downloadAnchor.click();
            downloadAnchor.remove();
        }

        window.onload = () => {
            applyTabAccess('');
            loadDepartmentList();
            loadEvaluationFormOptions();
        };
    "#;
