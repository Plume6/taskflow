// API 配置
// 在 Docker 环境下通过 nginx 反向代理，使用相对路径（/api/v1）
// 开发环境可在 HTML 中定义 __API_BASE_URL 覆盖此值，或直接修改下方代码
const API_BASE_URL = (typeof __API_BASE_URL !== 'undefined' ? __API_BASE_URL : '/api/v1');
let authToken = localStorage.getItem('authToken');
let currentUser = null;

// ========== 工具函数 ==========

// 显示提示消息
function showToast(message, type = 'info') {
    const toast = document.getElementById('toast');
    toast.textContent = message;
    toast.className = `toast ${type} show`;
    setTimeout(() => {
        toast.classList.remove('show');
    }, 3000);
}

// API 请求封装
async function apiRequest(endpoint, method, data = null, requiresAuth = true) {
    const url = `${API_BASE_URL}${endpoint}`;
    const headers = {};

    if (requiresAuth && authToken) {
        headers['Authorization'] = `Bearer ${authToken}`;
    }

    const options = {
        method,
        headers,
    };

    if (data !== null && data !== undefined) {
        headers['Content-Type'] = 'application/json';
        options.body = JSON.stringify(data);
    }
    
    try {
        const response = await fetch(url, options);
        const text = await response.text();
        let result = {};
        if (text.length > 0) {
            try {
                result = JSON.parse(text);
            } catch {
                if (!response.ok) {
                    throw new Error(text.slice(0, 200) || `请求失败 (${response.status})`);
                }
                throw new Error('服务器返回了无效数据');
            }
        }

        if (!response.ok) {
            const msg = result.error?.message || result.message || `请求失败 (${response.status})`;
            throw new Error(msg);
        }

        if (result && typeof result === 'object' && !Array.isArray(result)) {
            result.status = response.status;
        }
        return result;
    } catch (error) {
        console.error('API Error:', error);
        showToast(error.message, 'error');
        throw error;
    }
}

// ========== 认证相关 ==========

// 注册
async function register(username, email, password) {
    const result = await apiRequest('/auth/register', 'POST', {
        username,
        email,
        password,
    }, false);
    
    if (result.success) {
        showToast('注册成功！请登录', 'success');
        document.querySelector('[data-tab="login"]').click();
        return true;
    }
    return false;
}

// 登录
async function login(email, password) {
    const result = await apiRequest('/auth/login', 'POST', {
        email,
        password,
    }, false);
    
    if (result.success) {
        authToken = result.data.token;
        currentUser = result.data;
        localStorage.setItem('authToken', authToken);
        localStorage.setItem('currentUser', JSON.stringify(currentUser));
        showToast('登录成功！', 'success');
        loadApp();
        return true;
    }
    return false;
}

// 退出登录
function logout() {
    authToken = null;
    currentUser = null;
    localStorage.removeItem('authToken');
    localStorage.removeItem('currentUser');
    showToast('已退出登录', 'info');
    showAuthSection();
}

// 检查登录状态
function checkAuth() {
    const storedToken = localStorage.getItem('authToken');
    const storedUser = localStorage.getItem('currentUser');
    
    if (storedToken && storedUser) {
        authToken = storedToken;
        currentUser = JSON.parse(storedUser);
        loadApp();
    } else {
        showAuthSection();
    }
}

// ========== 任务相关 ==========

// 加载任务列表
async function loadTasks() {
    try {
        const result = await apiRequest('/tasks', 'GET');
        if (result.success) {
            renderTasks(result.data);
            return result.data;
        }
    } catch (error) {
        console.error('Load tasks error:', error);
        return [];
    }
}

// 创建任务
async function createTask(title, description) {
    const result = await apiRequest('/tasks', 'POST', {
        title,
        description: description || null,
    });
    
    if (result.success) {
        showToast('任务创建成功', 'success');
        loadTasks();
        loadStats();
        return true;
    }
    return false;
}

// 更新任务
async function updateTask(taskId, data) {
    const result = await apiRequest(`/tasks/${taskId}`, 'PUT', data);
    if (result.success) {
        showToast('任务更新成功', 'success');
        loadTasks();
        loadStats();
        return true;
    }
    return false;
}

// 删除任务
async function deleteTask(taskId) {
    if (!confirm('确定要删除这个任务吗？')) return false;

    try {
        const result = await apiRequest(`/tasks/${taskId}`, 'DELETE');
        if (result.success) {
            showToast('任务删除成功', 'success');
            loadTasks();
            loadStats();
            return true;
        }
    } catch (error) {
        console.error('Delete task error:', error);
        return false;
    }
    return false;
}

// 加载统计数据
async function loadStats() {
    try {
        const result = await apiRequest('/tasks/stats', 'GET');
        if (result.success) {
            const stats = result.data;
            document.getElementById('statTotal').textContent = stats.total || 0;
            document.getElementById('statPending').textContent = stats.pending || 0;
            document.getElementById('statInProgress').textContent = stats.in_progress || 0;
            document.getElementById('statCompleted').textContent = stats.completed || 0;
        }
    } catch (error) {
        console.error('Load stats error:', error);
    }
}

// ========== 渲染函数 ==========

/** 统一状态键（兼容历史大小写、首尾空格） */
function normalizeTaskStatus(status) {
    if (status == null) return 'pending';
    const raw = String(status).trim();
    const s = raw.replace(/([A-Z])/g, '_$1').toLowerCase().replace(/^_/, '');
    if (s === 'pending') return 'pending';
    if (s === 'in_progress' || s === 'inprogress') return 'in_progress';
    if (s === 'completed') return 'completed';
    return 'pending';
}

/** 是否展示某操作（优先用后端 available_actions，兼容旧接口） */
function taskAllows(task, action, st) {
    const list = task.available_actions;
    if (Array.isArray(list) && list.length) {
        return list.includes(action);
    }
    if (action === 'start') return st === 'pending';
    if (action === 'complete') return st !== 'completed';
    if (action === 'delete') return true;
    return false;
}

// 渲染任务列表
function renderTasks(tasks) {
    const taskList = document.getElementById('taskList');
    const taskCount = document.getElementById('taskCount');
    
    taskCount.textContent = tasks.length;
    
    if (!tasks.length) {
        taskList.innerHTML = '<div class="empty-state">✨ 暂无任务，创建一个吧！</div>';
        return;
    }
    
    const statusMap = {
        'pending': { text: '待处理', class: 'status-pending' },
        'in_progress': { text: '进行中', class: 'status-progress' },
        'completed': { text: '已完成', class: 'status-completed' }
    };
    
    taskList.innerHTML = tasks.map(task => {
        const st = normalizeTaskStatus(task.status);
        const sid = escapeHtml(String(task.id));
        const startBtn = taskAllows(task, 'start', st)
            ? `<button type="button" class="btn-task-action start" onclick="markInProgress('${sid}')" title="设为进行中">开始</button>`
            : '';
        const completeBtn = taskAllows(task, 'complete', st)
            ? `<button type="button" class="btn-task-action complete" onclick="markComplete('${sid}')" title="标记完成">完成</button>`
            : '';
        const deleteBtn = taskAllows(task, 'delete', st)
            ? `<button type="button" class="btn-task-action danger" onclick="deleteTaskHandler('${sid}')" title="删除">删除</button>`
            : '';
        return `
        <div class="task-item" data-id="${sid}">
            <div class="task-content">
                <div class="task-title">${escapeHtml(task.title)}</div>
                ${task.description ? `<div class="task-desc">${escapeHtml(task.description)}</div>` : ''}
                <div class="task-meta">
                    <span>创建于: ${new Date(task.created_at).toLocaleString()}</span>
                    <span class="task-status ${statusMap[st]?.class || 'status-pending'}">
                        ${statusMap[st]?.text || st}
                    </span>
                </div>
            </div>
            <div class="task-actions">
                ${startBtn}
                ${completeBtn}
                ${deleteBtn}
            </div>
        </div>`;
    }).join('');
}

// 设为进行中
async function markInProgress(taskId) {
    await updateTask(taskId, { status: 'in_progress' });
}

// 标记任务完成
async function markComplete(taskId) {
    await updateTask(taskId, { status: 'completed' });
}

// 全局删除任务处理
async function deleteTaskHandler(taskId) {
    await deleteTask(taskId);
}

// 转义 HTML 防止 XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ========== UI 切换 ==========

// 显示认证区域
function showAuthSection() {
    document.getElementById('authSection').style.display = 'block';
    document.getElementById('appSection').style.display = 'none';
    document.getElementById('logoutBtn').style.display = 'none';
    
    // 清空表单
    document.getElementById('loginEmail').value = '';
    document.getElementById('loginPassword').value = '';
    document.getElementById('regUsername').value = '';
    document.getElementById('regEmail').value = '';
    document.getElementById('regPassword').value = '';
}

// 加载应用主界面
async function loadApp() {
    document.getElementById('authSection').style.display = 'none';
    document.getElementById('appSection').style.display = 'block';
    document.getElementById('logoutBtn').style.display = 'block';
    
    // 显示用户信息
    if (currentUser) {
        document.getElementById('username').textContent = currentUser.username;
    }
    
    // 加载数据
    await loadTasks();
    await loadStats();
}

// ========== 事件绑定 ==========

function bindEvents() {
    // Tab 切换
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const tab = btn.dataset.tab;
            document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            
            document.querySelectorAll('.auth-form').forEach(form => form.classList.remove('active'));
            if (tab === 'login') {
                document.getElementById('loginForm').classList.add('active');
            } else {
                document.getElementById('registerForm').classList.add('active');
            }
        });
    });
    
    // 登录
    document.getElementById('loginBtn').addEventListener('click', async () => {
        const email = document.getElementById('loginEmail').value.trim();
        const password = document.getElementById('loginPassword').value;
        
        if (!email || !password) {
            showToast('请填写邮箱和密码', 'error');
            return;
        }
        
        await login(email, password);
    });
    
    // 注册
    document.getElementById('registerBtn').addEventListener('click', async () => {
        const username = document.getElementById('regUsername').value.trim();
        const email = document.getElementById('regEmail').value.trim();
        const password = document.getElementById('regPassword').value;
        
        if (!username || !email || !password) {
            showToast('请填写完整信息', 'error');
            return;
        }
        
        if (password.length < 6) {
            showToast('密码至少需要6位', 'error');
            return;
        }
        
        await register(username, email, password);
    });
    
    // 创建任务
    document.getElementById('createTaskBtn').addEventListener('click', async () => {
        const title = document.getElementById('taskTitle').value.trim();
        const description = document.getElementById('taskDesc').value.trim();
        
        if (!title) {
            showToast('请填写任务标题', 'error');
            return;
        }
        
        const success = await createTask(title, description);
        if (success) {
            document.getElementById('taskTitle').value = '';
            document.getElementById('taskDesc').value = '';
        }
    });
    
    // 退出登录
    document.getElementById('logoutBtn').addEventListener('click', logout);
    
    // 回车提交
    document.getElementById('loginPassword').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') document.getElementById('loginBtn').click();
    });
    
    document.getElementById('regPassword').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') document.getElementById('registerBtn').click();
    });
    
    document.getElementById('taskTitle').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') document.getElementById('createTaskBtn').click();
    });
}

// ========== 初始化 ==========

document.addEventListener('DOMContentLoaded', () => {
    bindEvents();
    checkAuth();
});

// 暴露全局函数供 HTML 调用
window.markInProgress = markInProgress;
window.markComplete = markComplete;
window.deleteTaskHandler = deleteTaskHandler;