local CanonicalInstance = require(script.CanonicalInstance)

return {
	EncodedValue = require(script.EncodedValue),
	CanonicalProperty = require(script.CanonicalProperty),
	CanonicalInstance = CanonicalInstance,
	PropertySelection = CanonicalInstance.PropertySelection,
}