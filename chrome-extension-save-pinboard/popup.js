document.addEventListener('DOMContentLoaded', async () => {
  const apiTokenInput = document.getElementById('api-token');
  const urlInput = document.getElementById('url');
  const titleInput = document.getElementById('title');
  const descriptionInput = document.getElementById('description');
  const tagsInput = document.getElementById('tags');
  const privateCheckbox = document.getElementById('private');
  const toreadCheckbox = document.getElementById('toread');
  const saveBtn = document.getElementById('save-btn');
  const cancelBtn = document.getElementById('cancel-btn');
  const statusDiv = document.getElementById('status');

  // Load saved API token
  const storage = await chrome.storage.sync.get(['pinboardApiToken']);
  if (storage.pinboardApiToken) {
    apiTokenInput.value = storage.pinboardApiToken;
  }

  // Get current tab info
  try {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    urlInput.value = tab.url;
    titleInput.value = tab.title;
  } catch (error) {
    console.error('Failed to get tab info:', error);
  }

  // Save button click handler
  saveBtn.addEventListener('click', async () => {
    const apiToken = apiTokenInput.value.trim();
    const url = urlInput.value.trim();
    const title = titleInput.value.trim();
    const description = descriptionInput.value.trim();
    const tags = tagsInput.value.trim();
    const isPrivate = privateCheckbox.checked;
    const isToRead = toreadCheckbox.checked;

    if (!apiToken) {
      showStatus('API token is required', 'error');
      return;
    }

    if (!url) {
      showStatus('URL is required', 'error');
      return;
    }

    if (!title) {
      showStatus('Title is required', 'error');
      return;
    }

    // Save API token for future use
    await chrome.storage.sync.set({ pinboardApiToken: apiToken });

    // Show loading status
    showStatus('Saving to Pinboard...', 'loading');
    saveBtn.disabled = true;

    try {
      await saveToPinboard(apiToken, url, title, description, tags, isPrivate, isToRead);
      showStatus('Successfully saved to Pinboard!', 'success');
      
      // Close popup after a short delay
      setTimeout(() => {
        window.close();
      }, 1500);
    } catch (error) {
      showStatus(`Error: ${error.message}`, 'error');
      saveBtn.disabled = false;
    }
  });

  // Cancel button click handler
  cancelBtn.addEventListener('click', () => {
    window.close();
  });

  // Show status message
  function showStatus(message, type) {
    statusDiv.textContent = message;
    statusDiv.className = `status ${type}`;
    statusDiv.style.display = 'block';
  }

  // Save bookmark to Pinboard
  async function saveToPinboard(apiToken, url, title, description, tags, isPrivate, isToRead) {
    const params = new URLSearchParams({
      auth_token: apiToken,
      url: url,
      description: title,
      extended: description,
      tags: tags,
      shared: isPrivate ? 'no' : 'yes',
      toread: isToRead ? 'yes' : 'no',
      replace: 'yes',
      format: 'json'
    });

    const response = await fetch(`https://api.pinboard.in/v1/posts/add?${params}`, {
      method: 'GET',
      headers: {
        'User-Agent': 'PinboardSaver/1.0'
      }
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const result = await response.json();
    
    if (result.result_code !== 'done') {
      throw new Error(result.result_code || 'Unknown error occurred');
    }

    return result;
  }
});