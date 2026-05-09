export function toFileInfo(path) {
  return {
    path,
    name: path.split(/[/\\]/).pop()
  }
}

export function withPendingStatus(file) {
  return {
    ...file,
    status: 'pending',
    progress: 0,
    error: null
  }
}

export function addUniqueQueuedFiles(files, newFiles) {
  const existingPaths = new Set(files.map(file => file.path))
  const queuedFiles = newFiles
    .filter(file => !existingPaths.has(file.path))
    .map(withPendingStatus)

  return [...files, ...queuedFiles]
}

export function updateFileProgress(files, payload) {
  const { path, status, progress, error } = payload

  return files.map(file => {
    if (file.path !== path) return file

    return {
      ...file,
      status,
      progress: progress || 0,
      error: error || null
    }
  })
}

export function resetFilesForProcessing(files, paths) {
  const processingPaths = new Set(paths)

  return files.map(file => {
    if (!processingPaths.has(file.path) || file.status === 'success') return file

    return {
      ...file,
      status: 'pending',
      progress: 0,
      error: null
    }
  })
}

export function pendingFilePaths(files) {
  return files
    .filter(file => file.status !== 'success')
    .map(file => file.path)
}

export function clearFinishedFiles(files) {
  return files.filter(file => file.status !== 'success' && file.status !== 'error')
}
