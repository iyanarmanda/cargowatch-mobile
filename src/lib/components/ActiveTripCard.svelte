<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  export interface ActiveTrip {
    tripId: string;
    driverName: string;
    plateNumber: string;
    deviceId: string;
    startedAt: string;
    origin?: string;
    destination: string;
    driverId: string;
  }

  export let apiBaseUrl: string;
  export let activeTrip: ActiveTrip;
  export let onTripEnded: () => void;
  export let onError: (message: string) => void;

  let isLoading = false;
  let isCancelling = false;
  let showCancelModal = false;
  let cancelReason = '';
  let successMessage = '';

  function clearMessages() {
    successMessage = '';
    onError('');
  }

  async function handleCompleteTrip() {
    isLoading = true;
    clearMessages();

    try {
      await invoke<any>('patch_request', {
        url: `${apiBaseUrl}/trips/${activeTrip.driverId}/complete`,
        payload: {
          tripId: activeTrip.tripId || '',
        }
      });

      successMessage = 'Trip completed successfully!';
      setTimeout(() => {
        onTripEnded();
      }, 800);
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : (err?.message || 'Failed to complete trip.');
      onError(msg);
    } finally {
      isLoading = false;
    }
  }

  async function handleCancelTrip() {
    if (!cancelReason.trim()) {
      onError('Trip cancel reason is required.');
      return;
    }

    isCancelling = true;
    clearMessages();

    try {
      await invoke('patch_request', {
        url: `${apiBaseUrl}/trips/${activeTrip.tripId}/cancel`,
        payload: {
          reason: cancelReason,
          cancelledBy: 'driver'
        }
      });

      successMessage = 'Trip cancelled successfully.';
      showCancelModal = false;
      setTimeout(() => {
        onTripEnded();
      }, 800);
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : (err?.message || 'Failed to cancelled trip.');
      onError(msg);
    } finally {
      isCancelling = false;
    }
  }
</script>

<div class="card active-card">
  <div class="header-status">
    <span class="status-badge pulse">🟢 ON GOING</span>
    <small class="time-stamp">Start at: {activeTrip.startedAt}</small>
  </div>

  <h3>Active Trip Status</h3>

  {#if successMessage}
    <div class="alert success">
      {successMessage}
    </div>
  {/if}

  <div class="info-group">
    <div class="route-box">
      <p><strong>Origin:</strong> {activeTrip.origin || 'Surabaya'}</p>
      <p><strong>Destination:</strong> {activeTrip.destination}</p>
    </div>
    <hr class="divider" />
    <p><strong>Driver:</strong> {activeTrip.driverName}</p>
    <p><strong>Plate License:</strong> {activeTrip.plateNumber}</p>
    <p><strong>IoT Device ID:</strong> {activeTrip.deviceId}</p>
  </div>

  <div class="action-buttons">
    <button class="btn success-btn" on:click={handleCompleteTrip} disabled={isLoading || isCancelling}>
      {isLoading ? 'Processing...' : 'Complete Trip'}
    </button>
    <button class="btn danger-outline" on:click={() => (showCancelModal = true)} disabled={isLoading || isCancelling}>
      Cancel Trip
    </button>
  </div>
</div>

{#if showCancelModal}
  <div class="modal-overlay">
    <div class="modal-content">
      <h4>Emergency Cancel a Trip</h4>
      <p>Input trip cancel reason:</p>

      <textarea
        bind:value={cancelReason}
        placeholder="Example: Broken-down truck / Bad weather / Traffic accident"
        rows="3"
      ></textarea>

      <div class="modal-actions">
        <button class="btn secondary" on:click={() => (showCancelModal = false)} disabled={isCancelling}>
          Cancel
        </button>
        <button class="btn danger" on:click={handleCancelTrip} disabled={isCancelling}>
          {isCancelling ? 'Cancelling...' : 'Confirm'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .active-card {
    background: #ffffff;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .header-status {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .status-badge {
    background: #dcfce7;
    color: #15803d;
    padding: 4px 10px;
    border-radius: 20px;
    font-size: 0.8em;
    font-weight: bold;
  }

  .time-stamp {
    color: #64748b;
    font-size: 0.8em;
  }

  .route-box {
    background: #f8fafc;
    padding: 10px 14px;
    border-radius: 6px;
    border-left: 3px solid #0284c7;
    margin-bottom: 12px;
  }

  .divider {
    border: none;
    border-top: 1px dashed #e2e8f0;
    margin: 12px 0;
  }

  .info-group p {
    margin: 6px 0;
    font-size: 0.95em;
  }

  .action-buttons {
    display: flex;
    gap: 10px;
    margin-top: 20px;
  }

  .btn {
    flex: 1;
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.9em;
    font-weight: 600;
    cursor: pointer;
    border: none;
  }

  .success-btn {
    background-color: #16a34a;
    color: white;
  }

  .danger-outline {
    background-color: transparent;
    color: #dc2626;
    border: 1px solid #dc2626;
  }

  .alert {
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.9em;
    margin-bottom: 14px;
  }

  .alert.success {
    background-color: #dcfce7;
    color: #15803d;
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    padding: 20px;
    border-radius: 8px;
    width: 90%;
    max-width: 400px;
  }

  textarea {
    width: 100%;
    padding: 8px;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    margin-top: 10px;
    margin-bottom: 15px;
    box-sizing: border-box;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  .btn.secondary {
    background: #e2e8f0;
    color: #334155;
  }

  .btn.danger {
    background: #dc2626;
    color: white;
  }
</style>
