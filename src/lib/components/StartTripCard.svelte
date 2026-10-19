<script lang="ts">
  import { onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Html5Qrcode } from 'html5-qrcode';

  export interface PendingTrip {
    id: string;
    origin?: string;
    destination: string;
    truckId: string;
    driverId: string;
    status: string;
  }

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
  export let driverId: string;
  export let deviceToken: string;
  export let pendingTrip: PendingTrip | null;
  export let onTripStarted: (trip: ActiveTrip) => void;
  export let onError: ((message: string) => void) | undefined = undefined;

  let isScanning = false;
  let isLoading = false;
  let successMessage = '';
  let scannerInstance: Html5Qrcode | null = null;

  const TRUCK_QR_REGEX = /^urn:cargowatch:truck:v1:([0-9a-fA-F-]{36})$/;

  onDestroy(async () => {
    await stopQrScanner();
  });

  function clearMessages() {
    successMessage = '';
    if (onError) onError('');
  }

  function setError(msg: string) {
    if (onError) onError(msg);
  }

  function extractTruckIdFromQr(decodedText: string): string | null {
    const cleanText = decodedText.trim();
    const match = cleanText.match(TRUCK_QR_REGEX);

    if (match && match[1]) {
      return match[1];
    }

    return null;
  }

  async function startQrScanner() {
    if (!pendingTrip) {
      setError('No confirmed task.');
      return;
    }

    isScanning = true;
    clearMessages();

    setTimeout(async () => {
      try {
        scannerInstance = new Html5Qrcode('qr-reader');
        await scannerInstance.start(
          { facingMode: 'environment' },
          { fps: 10, qrbox: { width: 250, height: 250 } },
          onScanSuccess,
          () => {}
        );
      } catch (err) {
        console.error(err);
        setError('Failed to open camera. Make sure camera access is granted.');
        isScanning = false;
      }
    }, 100);
  }

  async function stopQrScanner() {
    if (scannerInstance && scannerInstance.isScanning) {
      await scannerInstance.stop();
      scannerInstance.clear();
    }
    isScanning = false;
  }

  async function onScanSuccess(decodedText: string) {
    await stopQrScanner();

    const scannedTruckId = extractTruckIdFromQr(decodedText);

    if (!scannedTruckId) {
      setError('QR code not valid! Make sure you scan qr code from CargoWatch fleet.');
      return;
    }

    await handleConfirmTrip(scannedTruckId);
  }

  async function handleConfirmTrip(scannedTruckId: string) {
    if (!pendingTrip) return;

    isLoading = true;
    clearMessages();

    try {
      const result = await invoke<any>('patch_request', {
        url: `${apiBaseUrl}/trips/${pendingTrip.id}/confirm-start`,
        payload: {
          driverId: driverId,
          truckId: scannedTruckId,
          mobileToken: deviceToken
        }
      });

      const responseData = result.data || result;
      successMessage = 'Trip successfully confirmed! Starting session...';

      const newTrip: ActiveTrip = {
        tripId: responseData.id || pendingTrip.id,
        driverName: 'Driver',
        plateNumber: responseData.plateNumber,
        deviceId: responseData.deviceId,
        origin: responseData.origin || 'Surabaya',
        destination: responseData.destination,
        startedAt: new Date().toLocaleTimeString('id-ID')
      };

      setTimeout(() => {
        onTripStarted(newTrip);
      }, 800);

    } catch (err: any) {
      const msg = typeof err === 'string' ? err : (err?.message || 'Failed to start trip. Server returned an error.');
      setError(msg);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="card">
  <h3>Today Task</h3>

  {#if successMessage}
    <div class="alert success">
      {successMessage}
    </div>
  {/if}

  {#if pendingTrip}
    <div class="trip-info">
      <p><strong>Origin:</strong> {pendingTrip.origin || 'Surabaya'}</p>
      <p><strong>Destination:</strong> {pendingTrip.destination}</p>
      <p><strong>Status:</strong> <span class="badge">{pendingTrip.status}</span></p>
    </div>

    {#if !isScanning}
      <button class="btn primary" on:click={startQrScanner} disabled={isLoading}>
        {isLoading ? 'Processing...' : 'Scan QR & Start trip'}
      </button>
    {:else}
      <div class="scanner-wrapper">
        <div id="qr-reader"></div>
        <button class="btn danger" on:click={stopQrScanner}>Cancel Scan</button>
      </div>
    {/if}
  {:else}
    <p class="empty-state">Today is free.</p>
  {/if}
</div>

<style>
 .trip-info {
    margin-bottom: 20px;
    line-height: 1.6;
  }
  .badge {
    background: #e0f2fe;
    color: #0369a1;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 0.85em;
  }
  .empty-state {
    color: #666;
    font-style: italic;
  }

  .alert {
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.9em;
    margin-bottom: 16px;
    line-height: 1.4;
  }

  .alert.success {
    background-color: #dcfce7;
    color: #15803d;
    border: 1px solid #bbf7d0;
  }

  .scanner-wrapper {
    margin-top: 15px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
