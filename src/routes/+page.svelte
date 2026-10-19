<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification
  } from '@tauri-apps/plugin-notification';
  import StartTripCard from '$lib/components/StartTripCard.svelte';
  import ActiveTripCard from '$lib/components/ActiveTripCard.svelte';
  import '$lib/styles/global.css';

  import type { ActiveTrip, PendingTrip } from '$lib/components/StartTripCard.svelte';

  const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

  // for temporary
  const HARDCODED_DRIVER_ID = 'ad4a391f-4a4e-4574-9f27-29231bbeb357'

  let deviceToken = '';
  let pendingTrip: PendingTrip | null = null;
  let activeTrip: ActiveTrip | null = null;
  let errorMessage = '';
  let isFetchingTrip = false;
  let isCheckingState = false;
  let pollInterval = null;

  onMount(async () => {
    await Promise.all([
      initNotification(),
      checkDriverTripState(),
    ]);

    pollInterval = setInterval(() => {
      if (!activeTrip && !pendingTrip) {
        checkDriverTripState(true);
      }
    }, 5000);
  });

  onDestroy(() => {
    if (pollInterval) clearInterval(pollInterval);
  });

  async function initNotification() {
    try {
      let permission = await isPermissionGranted();
      if (!permission) {
        const permissionResponse = await requestPermission();
        permission = permissionResponse === 'granted';
      }
      if (!permission) {
        console.warn('Notification permission rejected by user');
        return;
      }

      const token = await invoke<string>('get_fcm_token');

      if (token) {
        deviceToken = token;
        localStorage.setItem('device_token', token);
        console.log('FCM token obtained:', deviceToken);
      }
    } catch (err) {
      console.error('Failed to get FCM token:', err);
    }
  }

  function parseTripData(res: any): any | null {
    if (!res) return null;
    
    if (res.statusCode && res.statusCode >= 400) return null;

    if (res.error || res.message === 'NotFoundException') return null;

    if (res.data && typeof res.data === 'object' && res.data.id) {
      return res.data;
    }
    
    if (res.id) {
      return res;
    }

    return null;
  }

  async function checkDriverTripState(isSilent = false) {
    if (isCheckingState) return;
    isCheckingState = true;

    if (!isSilent) isFetchingTrip = true;
    errorMessage = '';

    try { 
      const activeUrl = `${API_BASE_URL}/trips/driver/${HARDCODED_DRIVER_ID}/active`;
      const pendingUrl = `${API_BASE_URL}/trips/driver/${HARDCODED_DRIVER_ID}/pending`;

      const [activeResult, pendingResult] = await Promise.allSettled([
        invoke<any>('get_request', { url: activeUrl }),
        invoke<any>('get_request', { url: pendingUrl }),
      ]);

      let foundActive = false;
      if (activeResult.status === 'fulfilled') {
        const activeData = parseTripData(activeResult.value);
        if (activeData && activeData.id) {
          activeTrip = {
            tripId: activeData.id,
            driverId: HARDCODED_DRIVER_ID,
            driverName: activeData.driver?.name || 'Driver',
            plateNumber: activeData.truck?.plateNumber || '-',
            deviceId: activeData.truck?.deviceId || '-',
            origin: activeData.origin || 'Surabaya',
            destination: activeData.destination || '-',
            startedAt: activeData.startedAt
              ? new Date(activeData.startedAt).toLocaleTimeString('id-ID')
              : new Date().toLocaleTimeString('id-ID')
          };
          pendingTrip = null;
          foundActive = true;
        }
      }

      if (!foundActive) {
        activeTrip = null;

        if (pendingResult.status === 'fulfilled') {
          pendingTrip = parseTripData(pendingResult.value);
        } else {
          console.error('Fetch pending trip error:', pendingResult.reason);
        }
      }
    } catch (err) {
      console.error('Error checking trip state:', err);
    } finally {
      isCheckingState = false;
      if (!isSilent) isFetchingTrip = false;
    }
  }

  async function handleTripStarted(trip: ActiveTrip) {
    await checkDriverTripState();
  }

  function handleTripEnded() {
    activeTrip = null;
    checkDriverTripState();
  }

  function handleError(msg: string) {
    errorMessage = msg;
  }
</script>

<main class="container">
  <h2 class="title">CargoWatch Transport Monitor</h2>

  {#if errorMessage}
    <div class="alert error">
      {errorMessage}
    </div>
  {/if}

  {#if isFetchingTrip}
    <p>Loading Trip...</p>
  {:else if !activeTrip}
    <StartTripCard
      apiBaseUrl={API_BASE_URL}
      driverId={HARDCODED_DRIVER_ID}
      {deviceToken}
      {pendingTrip}
      onTripStarted={handleTripStarted}
      onError={handleError}
    />
  {:else}
    <ActiveTripCard
      apiBaseUrl={API_BASE_URL}
      {activeTrip}
      onTripEnded={handleTripEnded}
      onError={handleError}
    />
  {/if}
</main>

<style>
  .title {
    margin-top: 40px;
    margin-bottom: 10px;
  }
</style>
