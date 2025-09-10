// Main JavaScript for Secure Disk Erasure Tool GUI
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/api/fs';

class SecureDiskErasureGUI {
    constructor() {
        this.currentTab = 'devices';
        this.selectedDevice = null;
        this.devices = [];
        this.certificates = [];
        this.isWiping = false;
        
        this.init();
    }

    async init() {
        this.setupEventListeners();
        await this.loadSystemInfo();
        await this.loadDevices();
        this.setupTabNavigation();
    }

    setupEventListeners() {
        // Tab navigation
        document.querySelectorAll('[data-tab]').forEach(tab => {
            tab.addEventListener('click', (e) => {
                e.preventDefault();
                this.switchTab(e.target.dataset.tab);
            });
        });

        // Device refresh
        document.getElementById('refresh-devices').addEventListener('click', () => {
            this.loadDevices();
        });

        // Wipe form
        document.getElementById('wipe-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.showWipeConfirmation();
        });

        // Certificate verification
        document.getElementById('verify-btn').addEventListener('click', () => {
            this.verifyCertificate();
        });

        // Generate keys
        document.getElementById('generate-keys').addEventListener('click', () => {
            this.generateKeys();
        });

        // Settings
        document.getElementById('browse-output').addEventListener('click', () => {
            this.browseDirectory('output-directory');
        });

        // Confirm wipe
        document.getElementById('confirm-wipe-btn').addEventListener('click', () => {
            this.startWipeOperation();
        });
    }

    setupTabNavigation() {
        // Initialize with devices tab active
        this.switchTab('devices');
    }

    switchTab(tabName) {
        // Update navigation
        document.querySelectorAll('[data-tab]').forEach(tab => {
            tab.classList.remove('active');
        });
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

        // Update content
        document.querySelectorAll('.tab-content').forEach(content => {
            content.style.display = 'none';
        });
        document.getElementById(`${tabName}-tab`).style.display = 'block';

        this.currentTab = tabName;

        // Load tab-specific data
        switch (tabName) {
            case 'devices':
                this.loadDevices();
                break;
            case 'certificates':
                this.loadCertificates();
                break;
            case 'settings':
                this.loadSettings();
                break;
        }
    }

    async loadSystemInfo() {
        try {
            const systemInfo = await invoke('get_system_info');
            document.getElementById('os-info').textContent = systemInfo.os;
            document.getElementById('arch-info').textContent = systemInfo.arch;
        } catch (error) {
            console.error('Failed to load system info:', error);
        }
    }

    async loadDevices() {
        try {
            this.showLoading('devices-container');
            const devices = await invoke('list_devices', { detailed: true });
            this.devices = devices;
            this.renderDevices(devices);
        } catch (error) {
            console.error('Failed to load devices:', error);
            this.showError('devices-container', 'Failed to load devices. Make sure you have the necessary permissions.');
        }
    }

    renderDevices(devices) {
        const container = document.getElementById('devices-container');
        const noDevicesAlert = document.getElementById('no-devices');

        if (devices.length === 0) {
            container.innerHTML = '';
            noDevicesAlert.style.display = 'block';
            return;
        }

        noDevicesAlert.style.display = 'none';
        container.innerHTML = devices.map(device => `
            <div class="col-md-6 col-lg-4 mb-3">
                <div class="card device-card" data-device-path="${device.path}">
                    <div class="card-body">
                        <div class="d-flex justify-content-between align-items-start mb-2">
                            <h6 class="card-title mb-0">${device.name}</h6>
                            <span class="badge bg-${this.getDeviceTypeColor(device.device_type)}">${device.device_type}</span>
                        </div>
                        <p class="card-text">
                            <small class="text-muted">
                                <div><i class="bi bi-hdd me-1"></i> ${this.formatBytes(device.size)}</div>
                                <div><i class="bi bi-geo-alt me-1"></i> ${device.path}</div>
                                ${device.model ? `<div><i class="bi bi-tag me-1"></i> ${device.model}</div>` : ''}
                                ${device.serial ? `<div><i class="bi bi-upc me-1"></i> ${device.serial}</div>` : ''}
                            </small>
                        </p>
                        <div class="mt-2">
                            <span class="status-indicator ${device.supports_secure_erase ? 'status-success' : 'status-danger'}"></span>
                            <small>Secure Erase</small>
                            <span class="status-indicator ${device.supports_trim ? 'status-success' : 'status-warning'} ms-3"></span>
                            <small>TRIM Support</small>
                        </div>
                        ${device.hidden_areas.length > 0 ? `
                            <div class="mt-2">
                                <small class="text-warning">
                                    <i class="bi bi-exclamation-triangle me-1"></i>
                                    ${device.hidden_areas.length} hidden area(s)
                                </small>
                            </div>
                        ` : ''}
                    </div>
                </div>
            </div>
        `).join('');

        // Add click handlers
        document.querySelectorAll('.device-card').forEach(card => {
            card.addEventListener('click', () => {
                this.selectDevice(card.dataset.devicePath);
            });
        });
    }

    selectDevice(devicePath) {
        // Update visual selection
        document.querySelectorAll('.device-card').forEach(card => {
            card.classList.remove('selected');
        });
        document.querySelector(`[data-device-path="${devicePath}"]`).classList.add('selected');

        // Update selected device
        this.selectedDevice = this.devices.find(d => d.path === devicePath);

        // Update wipe form
        const deviceSelect = document.getElementById('selected-device');
        deviceSelect.innerHTML = '<option value="">Select a device...</option>';
        this.devices.forEach(device => {
            const option = document.createElement('option');
            option.value = device.path;
            option.textContent = `${device.name} (${this.formatBytes(device.size)})`;
            if (device.path === devicePath) {
                option.selected = true;
            }
            deviceSelect.appendChild(option);
        });
    }

    getDeviceTypeColor(deviceType) {
        switch (deviceType) {
            case 'HDD': return 'secondary';
            case 'SSD': return 'primary';
            case 'NVMe': return 'success';
            case 'USB': return 'warning';
            default: return 'dark';
        }
    }

    formatBytes(bytes) {
        const units = ['B', 'KB', 'MB', 'GB', 'TB'];
        let size = bytes;
        let unitIndex = 0;

        while (size >= 1024 && unitIndex < units.length - 1) {
            size /= 1024;
            unitIndex++;
        }

        return `${size.toFixed(2)} ${units[unitIndex]}`;
    }

    showWipeConfirmation() {
        const devicePath = document.getElementById('selected-device').value;
        const wipeMode = document.getElementById('wipe-mode').value;
        const generateCertificate = document.getElementById('generate-certificate').checked;

        if (!devicePath) {
            this.showAlert('Please select a device to wipe.', 'warning');
            return;
        }

        const device = this.devices.find(d => d.path === devicePath);
        const detailsHtml = `
            <div class="row">
                <div class="col-6"><strong>Device:</strong></div>
                <div class="col-6">${device.name}</div>
            </div>
            <div class="row">
                <div class="col-6"><strong>Path:</strong></div>
                <div class="col-6">${device.path}</div>
            </div>
            <div class="row">
                <div class="col-6"><strong>Size:</strong></div>
                <div class="col-6">${this.formatBytes(device.size)}</div>
            </div>
            <div class="row">
                <div class="col-6"><strong>Mode:</strong></div>
                <div class="col-6">${wipeMode}</div>
            </div>
            <div class="row">
                <div class="col-6"><strong>Certificate:</strong></div>
                <div class="col-6">${generateCertificate ? 'Yes' : 'No'}</div>
            </div>
        `;

        document.getElementById('wipe-confirmation-details').innerHTML = detailsHtml;
        
        const modal = new bootstrap.Modal(document.getElementById('confirmWipeModal'));
        modal.show();
    }

    async startWipeOperation() {
        const devicePath = document.getElementById('selected-device').value;
        const wipeMode = document.getElementById('wipe-mode').value;
        const generateCertificate = document.getElementById('generate-certificate').checked;
        const verifyAfterWipe = document.getElementById('verify-after-wipe').checked;

        // Close modal
        const modal = bootstrap.Modal.getInstance(document.getElementById('confirmWipeModal'));
        modal.hide();

        // Show progress
        document.getElementById('progress-container').style.display = 'block';
        this.isWiping = true;

        // Update UI
        document.getElementById('start-wipe').disabled = true;
        document.getElementById('cancel-wipe').disabled = false;

        try {
            const result = await invoke('wipe_device', {
                device: devicePath,
                mode: wipeMode,
                certificate: generateCertificate,
                verify: verifyAfterWipe
            });

            this.showAlert('Wipe operation completed successfully!', 'success');
            this.logOperation('Wipe operation completed successfully');
            
            if (generateCertificate && result.certificatePath) {
                this.showAlert(`Certificate generated: ${result.certificatePath}`, 'info');
            }

        } catch (error) {
            this.showAlert(`Wipe operation failed: ${error}`, 'danger');
            this.logOperation(`Wipe operation failed: ${error}`);
        } finally {
            this.isWiping = false;
            document.getElementById('start-wipe').disabled = false;
            document.getElementById('cancel-wipe').disabled = true;
        }
    }

    async verifyCertificate() {
        const certificateFile = document.getElementById('certificate-file').files[0];
        const publicKeyFile = document.getElementById('public-key-file').files[0];

        if (!certificateFile) {
            this.showAlert('Please select a certificate file.', 'warning');
            return;
        }

        try {
            const certificatePath = certificateFile.path;
            const publicKeyPath = publicKeyFile ? publicKeyFile.path : null;

            const result = await invoke('verify_certificate', {
                certificate: certificatePath,
                publicKey: publicKeyPath
            });

            this.displayVerificationResults(result);

        } catch (error) {
            this.showAlert(`Certificate verification failed: ${error}`, 'danger');
        }
    }

    displayVerificationResults(result) {
        const container = document.getElementById('verification-results');
        
        const statusClass = result.is_valid ? 'success' : 'danger';
        const statusIcon = result.is_valid ? 'check-circle' : 'x-circle';
        
        container.innerHTML = `
            <div class="text-center mb-3">
                <i class="bi bi-${statusIcon} text-${statusClass}" style="font-size: 3rem;"></i>
                <h5 class="mt-2 text-${statusClass}">${result.is_valid ? 'Valid Certificate' : 'Invalid Certificate'}</h5>
            </div>
            
            <div class="row">
                <div class="col-6">
                    <div class="d-flex align-items-center mb-2">
                        <i class="bi bi-${result.signature_valid ? 'check' : 'x'}-circle text-${result.signature_valid ? 'success' : 'danger'} me-2"></i>
                        <span>Signature</span>
                    </div>
                    <div class="d-flex align-items-center mb-2">
                        <i class="bi bi-${result.hash_valid ? 'check' : 'x'}-circle text-${result.hash_valid ? 'success' : 'danger'} me-2"></i>
                        <span>Hash</span>
                    </div>
                    <div class="d-flex align-items-center mb-2">
                        <i class="bi bi-${result.compliance_valid ? 'check' : 'x'}-circle text-${result.compliance_valid ? 'success' : 'danger'} me-2"></i>
                        <span>Compliance</span>
                    </div>
                </div>
                <div class="col-6">
                    <small class="text-muted">
                        <div>Age: ${result.verification_details.certificate_age_days} days</div>
                        <div>Size: ${result.verification_details.device_size_gb} GB</div>
                        <div>Duration: ${result.verification_details.wipe_duration_seconds}s</div>
                    </small>
                </div>
            </div>
            
            ${result.warnings.length > 0 ? `
                <div class="mt-3">
                    <h6 class="text-warning">Warnings</h6>
                    <ul class="list-unstyled">
                        ${result.warnings.map(warning => `<li><i class="bi bi-exclamation-triangle me-1"></i>${warning}</li>`).join('')}
                    </ul>
                </div>
            ` : ''}
            
            ${result.errors.length > 0 ? `
                <div class="mt-3">
                    <h6 class="text-danger">Errors</h6>
                    <ul class="list-unstyled">
                        ${result.errors.map(error => `<li><i class="bi bi-x-circle me-1"></i>${error}</li>`).join('')}
                    </ul>
                </div>
            ` : ''}
        `;
    }

    async generateKeys() {
        try {
            const outputDir = await open({
                directory: true,
                title: 'Select output directory for keys'
            });

            if (outputDir) {
                await invoke('generate_keys', { output: outputDir });
                this.showAlert('Key pair generated successfully!', 'success');
            }
        } catch (error) {
            this.showAlert(`Failed to generate keys: ${error}`, 'danger');
        }
    }

    async browseDirectory(inputId) {
        try {
            const selectedDir = await open({
                directory: true,
                title: 'Select directory'
            });

            if (selectedDir) {
                document.getElementById(inputId).value = selectedDir;
            }
        } catch (error) {
            console.error('Failed to browse directory:', error);
        }
    }

    async loadCertificates() {
        // Load recent certificates from local storage or file system
        // This would be implemented based on the actual storage mechanism
    }

    async loadSettings() {
        // Load settings from local storage
        // This would be implemented based on the actual storage mechanism
    }

    logOperation(message) {
        const logContainer = document.getElementById('operation-log');
        const timestamp = new Date().toLocaleTimeString();
        const logEntry = document.createElement('div');
        logEntry.innerHTML = `<span class="text-muted">[${timestamp}]</span> ${message}`;
        logContainer.appendChild(logEntry);
        logContainer.scrollTop = logContainer.scrollHeight;
    }

    showLoading(containerId) {
        const container = document.getElementById(containerId);
        container.innerHTML = `
            <div class="text-center">
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">Loading...</span>
                </div>
                <p class="mt-2">Loading...</p>
            </div>
        `;
    }

    showError(containerId, message) {
        const container = document.getElementById(containerId);
        container.innerHTML = `
            <div class="alert alert-danger">
                <i class="bi bi-exclamation-triangle me-2"></i>
                ${message}
            </div>
        `;
    }

    showAlert(message, type = 'info') {
        // Create and show Bootstrap alert
        const alertDiv = document.createElement('div');
        alertDiv.className = `alert alert-${type} alert-dismissible fade show`;
        alertDiv.innerHTML = `
            ${message}
            <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
        `;
        
        // Insert at the top of the main content area
        const mainContent = document.querySelector('.col-md-9');
        mainContent.insertBefore(alertDiv, mainContent.firstChild);
        
        // Auto-dismiss after 5 seconds
        setTimeout(() => {
            if (alertDiv.parentNode) {
                alertDiv.remove();
            }
        }, 5000);
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new SecureDiskErasureGUI();
});
